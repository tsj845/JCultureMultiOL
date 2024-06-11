use crate::{common::*, rt::{Sockets, ErrCause}};
use tauri::{AppHandle, Manager};
use std::{any::{Any, TypeId}, env::args, thread};
use std::sync::mpsc::{Receiver, Sender, channel};
// use serde_json::Value as JSValue;
use std::sync::{Arc, Mutex, Condvar};

macro_rules! condvar {
    ($val:literal) => {
        Arc::new((Mutex::new($val), Condvar::new()))
    };
}

macro_rules! cvarpair {
    ($v1:ident, $v2:ident, $val:literal) => {
        $v1 = Arc::new((Mutex::new($val), Condvar::new()));
        $v2 = Arc::clone(&$v1);
    };
}

macro_rules! mutex {
    ($val:literal) => {
        Arc::new(Mutex::new($val))
    };
    ($val:expr) => {
        Arc::new(Mutex::new($val))
    };
}

macro_rules! mutexpair {
    ($m1:ident, $m2:ident, $val:literal) => {
        $m1 = Arc::new(Mutex::new($val));
        $m2 = Arc::clone(&$m1);
    };
    ($m1:ident, $m2:ident, $val:expr) => {
        $m1 = Arc::new(Mutex::new($val));
        $m2 = Arc::clone(&$m1);
    };
}

#[derive(Clone, Serialize, Deserialize)]
struct UIPlayerMovePayload {
    x: u32,
    y: u32
}

#[derive(Clone, Deserialize)]
struct PasswordPayload {
    canceled: bool,
    password: String
}

#[derive(Clone, Serialize, Deserialize)]
struct BoardInfo {
    w: u32,
    h: u32,
    tiles: Box<[u8]>,
    scores: [u32; 6]
}

#[derive(Clone, Serialize)]
struct BoardDims {
    w: u32,
    h: u32
}

#[derive(Clone, Serialize)]
struct PlayerInfo {
    name: String,
    pid: u16,
    tid: u8
}

#[derive(Clone, Serialize)]
struct MoveInfo {
    x: u32,
    y: u32,
    team: u8
}

enum GameState {
    Pregame,
    Midgame
}

// struct Password {
//     pub value: String
// }

// impl Password {
//     fn new() -> Self {
//         Self{value:String::new()}
//     }
// }

fn is_true(test:&str) -> bool{
    return test == "true" || test == "\"true\"";
}

fn confirm_server(app: &AppHandle, cdat: &ConnData) -> TResult<()> {
    let window = app.get_window("main").unwrap();
    let sdat = Sockets::connect_data(cdat)?;
    let (cv1, cv2);
    cvarpair!(cv1, cv2, false);
    let (val1, val2);
    mutexpair!(val1, val2, false);
    window.once("server-confirmed", move |event| {
        *(val2.lock().unwrap()) = is_true(event.payload().unwrap());
        *(cv2.0.lock().unwrap()) = true;
        cv2.1.notify_one();
    });
    window.emit("confirm-server", sdat)?;
    let mut confirmed = cv1.0.lock().unwrap();
    while !*confirmed {
        confirmed = cv1.1.wait(confirmed).unwrap();
    }
    if !*val1.lock().unwrap() {
        return Err(CancellationError::boxed());
    }
    Ok(())
}

/// if an `Ok(())` is returned, it is guaranteed that there either was no password, or there was and the user
/// entered it correctly
fn password_logic(app: &AppHandle, conn: &mut Sockets) -> TResult<()> {
    if !conn.check_password()? {
        return Ok(());
    }
    let window = app.get_window("main").unwrap();
    let mut reject = false;
    'outer: loop {
        let (cv1, cv2);
        cvarpair!(cv1, cv2, false);
        let (val1, val2);
        mutexpair!(val1, val2, String::new());
        window.once("password-input", move |event| {
            let pay: PasswordPayload = serde_json::from_str(event.payload().unwrap()).unwrap();
            // do appropriate thing based on if the prompt was canceled
            *(val2.lock().unwrap()) = match pay.canceled {
                false => pay.password.clone(),
                true => ("\0").to_string()
            };
            *(cv2.0.lock().unwrap()) = true;
            cv2.1.notify_one();
        });
        if reject {
            window.emit("password-reject", "")?;
        } else {
            window.emit("prompt-password", "")?;
        }
        let mut haspass = cv1.0.lock().unwrap();
        while !*haspass {
            haspass = cv1.1.wait(haspass).unwrap();
        }
        let attempt = (*val1.lock().unwrap()).clone();
        // check for cancellation
        if attempt.bytes().any(|b| b==0) {
            conn.decide_pw_continue(false)?;
            return Err(CancellationError::boxed());
        } else {
            conn.decide_pw_continue(true)?;
        }
        let accepted = conn.submit_password(&attempt)?;
        if accepted {
            window.emit("password-accept", "")?;
            break 'outer;
        }
        reject = true;
    }
    Ok(())
}

fn connect_server(app: &AppHandle, cdat: &ConnData, nickname: String) -> TResult<Sockets> {
    let window = app.get_window("main").unwrap();
    // confirm user wants to connect to this server
    confirm_server(app, cdat)?;
    // make that connection
    let mut conn = Sockets::connect_player(cdat)?;
    // check if there's a password, if so then do that logic too
    password_logic(app, &mut conn)?;
    conn.send_name(&nickname)?;
    return Ok(conn);
}

fn get_conn_data(app: &AppHandle) -> TResult<ConnData> {
    todo!()
}

pub fn entry(app: &mut tauri::App) -> TResult<()> {
    let window = app.get_window("main").unwrap();
    thread::spawn(move || {
        let (atx1, arx1): (Sender<String>, Receiver<String>) = channel();
        {
            // blocks until the window is ready
            let thtx = atx1.clone();
            window.once("window-ready", move |_| {
                thtx.send(String::new()).unwrap();
            });
        }
        arx1.recv().unwrap();
        println!("STARTED");
    });
    Ok(())
}

/*
let mut game_state = GameState::Pregame;
            let mut commcode: u8;
            loop {
                commcode = conn.read_byte().unwrap();
                match game_state {
                    GameState::Pregame => {
                        match commcode {
                            0 => {conn.shutdown();win2.trigger("close", None);return;},
                            1 => {
                                println!("STARTING GAME");
                                conn.write_byte(1).unwrap(); // spectating not supported currently
                                // let spc = conn.read_byte().unwrap();
                                // conn.read_k_bytes(spc as usize).unwrap();
                                let w = conn.read_size().unwrap();
                                let h = conn.read_size().unwrap();
                                let _ = conn.read_byte().unwrap();
                                win2.emit("create-board", BoardDims{w,h}).unwrap();
                                game_state = GameState::Midgame;
                            },
                            2 => {
                                let nl = conn.read_byte().unwrap();
                                let name = String::from_utf8(Vec::<u8>::from(conn.read_n_bytes(nl as usize).unwrap())).unwrap();
                                let pid = conn.read_player_id().unwrap();
                                let tid = conn.read_byte().unwrap();
                                win2.emit("player-join", PlayerInfo{name,pid,tid}).unwrap();
                            },
                            3 => {
                                let pid = conn.read_player_id().unwrap();
                                let tid = conn.read_byte().unwrap();
                                win2.emit("team-change", (pid, tid)).unwrap();
                            },
                            4 => {
                                win2.emit("player-leave", conn.read_byte().unwrap()).unwrap();
                            },
                            // not used for this client
                            5 => {let _ = conn.read_byte().unwrap();},
                            // not used for this client
                            6 => {
                                for _ in 0..conn.read_byte().unwrap() {
                                    for _ in 0..4 {
                                        let _ = conn.read_u16().unwrap();
                                    }
                                }
                            },
                            7 => {
                                let w = conn.read_size().unwrap();
                                let h = conn.read_size().unwrap();
                                let _ = conn.read_byte().unwrap();
                                let mut scores = [0u32; 6];
                                for i in 0..6 {
                                    scores[i] = conn.read_score().unwrap();
                                }
                                let mut tiles = byte_buf((w as usize)*(h as usize)*2);
                                for i in 0..(tiles.len()/2) {
                                    let sp = conn.read_bsp().unwrap();
                                    tiles[i*2] = sp.0;
                                    tiles[i*2+1] = sp.1;
                                }
                                win2.emit("full-board", BoardInfo{w,h,tiles,scores}).unwrap();
                            },
                            _ => {return;}
                        }
                    },
                    GameState::Midgame => {
                        if commcode == 1 {
                            // keep taking input until accepted
                            'outer: loop {
                                let inputready = condvar!(false);
                                let ir2 = inputready.clone();
                                let pmove = mutex!((0u32,0u32));
                                let pm2 = pmove.clone();
                                let (lock, cvar) = &*inputready;
                                let pmlock = &*pmove;
                                let mut hasinput = lock.lock().unwrap();
                                win2.once("board-input", move |event| {
                                    let (lock, cvar) = &*ir2;
                                    let lock2 = &*pm2;
                                    let pmp: UIPlayerMovePayload = serde_json::from_str(event.payload().unwrap()).unwrap();
                                    let mut pmv = lock2.lock().unwrap();
                                    *pmv = (pmp.x, pmp.y);
                                    *lock.lock().unwrap() = true;
                                    cvar.notify_one();
                                });
                                win2.emit("get-input", "").unwrap();
                                while !*hasinput {
                                    hasinput = cvar.wait(hasinput).unwrap();
                                }
                                let pm = (*pmlock.lock().unwrap()).clone();
                                match conn.try_move(pm.0, pm.1) {
                                    Ok(true) => {break 'outer;},
                                    Ok(false) => {},
                                    Err(_) => {conn.shutdown();thtx.send("ERROR{\"msg\":\"IO_ERROR\"}".to_string()).unwrap();return;}
                                }
                            }
                            let movecomplete = condvar!(false);
                            let mc2 = movecomplete.clone();
                            let gameover = mutex!(false);
                            let go2 = gameover.clone();
                            let (lock, cvar) = &*movecomplete;
                            let mut complete = lock.lock().unwrap();
                            win2.once("move-complete", move |event| {
                                let (lock, cvar) = &*mc2;
                                *lock.lock().unwrap() = true;
                                let mut go = go2.lock().unwrap();
                                *go = event.payload().unwrap() == "\"true\"";
                                cvar.notify_one();
                            });
                            let (movex, movey) = conn.read_move().unwrap();
                            let team = conn.read_team().unwrap();
                            win2.emit("player-move", MoveInfo{x:movex,y:movey,team}).unwrap();
                            while !*complete {
                                complete = cvar.wait(complete).unwrap();
                            }
                            if *gameover.lock().unwrap() {
                                game_state = GameState::Pregame;
                            }
                        }
                    }
                };
            }
*/