use crate::{common::*, rt::Connection};
use tauri::{AppHandle, Manager};
use std::thread;
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

#[derive(PartialEq)]
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

fn confirm_server(app: &AppHandle, cdat: &ConnData) -> TResult<ProtVer> {
    let window = app.get_window("main").unwrap();
    let sdat = Connection::connect_data(cdat)?;
    let ver = sdat.version.clone();
    if ver <= ProtVer(0, 1, 1) {
        return Err(VersionError::boxed(ProtVer(0,1,1), true));
    }
    println!("{:?}", &sdat);
    let (cv1, cv2);
    cvarpair!(cv1, cv2, false);
    let (val1, val2);
    mutexpair!(val1, val2, false);
    // let win2 = app.get_window("main").unwrap();
    // thread::spawn(move||{
        // win2.once("server-confirmed", move |event| {
        window.once("server-confirmed", move |event| {
            println!("CONFIRMED");
            // let _ = tx1.send(event.payload().unwrap().to_string());
            *(val2.lock().unwrap()) = is_true(event.payload().unwrap());
            *(cv2.0.lock().unwrap()) = true;
            cv2.1.notify_one();
        });
    // });
    window.emit("confirm-server", sdat)?;
    // let conf = is_true(&rx.recv()?);
    // println!("CONFIRMED: {}", conf);
    // if !conf {
    //     return Err(CancellationError::boxed());
    // }
    let mut confirmed = cv1.0.lock().unwrap();
    while !*confirmed {
        println!("UNCONFIRMED");
        confirmed = cv1.1.wait(confirmed).unwrap();
    }
    if !*val1.lock().unwrap() {
        return Err(CancellationError::boxed());
    }
    Ok(ver)
}

/// if an `Ok(())` is returned, it is guaranteed that there either was no password, or there was and the user
/// entered it correctly
fn password_logic(app: &AppHandle, conn: &mut Connection) -> TResult<()> {
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

fn connect_server(app: &AppHandle, cdat: &ConnData, nickname: String) -> TResult<Connection> {
    // confirm user wants to connect to this server
    let ver = confirm_server(app, cdat)?;
    // make that connection
    let mut conn = Connection::connect_player(cdat, ver)?;
    // let mut conn = Sockets::connect_player(cdat, [0, 1, 1])?;
    // check if there's a password, if so then do that logic too
    password_logic(app, &mut conn)?;
    conn.send_name(&nickname)?;
    conn.get_team()?;
    return Ok(conn);
}

fn runloop(app: &AppHandle, mut conn: Connection) -> TResult<()> {
    println!("RUNLOOP ENTRY");
    let window = app.get_window("main").unwrap();
    let win2 = app.get_window("main").unwrap();
    window.emit("join-server-result", JoinResultPayload{ok:true,msg:String::new()})?;
    let mut gamestate = GameState::Pregame;
    loop {
        let commcode: u8 = conn.read_byte()?;
        if gamestate == GameState::Pregame {
            match commcode {
                0 => {conn.shutdown();window.trigger("close", None);break;},
                1 => {
                    let w = conn.read_size()?;
                    let h = conn.read_size()?;
                    if conn.matches_version(ProtVer(0, 0, 1)) {
                        let _ = conn.read_size()?;
                    }
                    window.emit("create-board", BoardDims{w,h})?;
                    gamestate = GameState::Midgame;
                },
                2 => {
                    let nl = conn.read_byte()?;
                    let name = String::from_utf8(Vec::<u8>::from(conn.read_n_bytes(nl as usize)?)).unwrap();
                    let pid = conn.read_player_id()?;
                    let tid = conn.read_team()?;
                    win2.emit("player-join", PlayerInfo{name,pid,tid})?;
                },
                3 => {
                    let pid = conn.read_player_id()?;
                    let tid = conn.read_team()?;
                    win2.emit("team-change", (pid, tid))?;
                },
                4 => {
                    win2.emit("player-leave", conn.read_player_id()?)?;
                },
                // not used for this client
                5 => {let _ = conn.read_byte()?;},
                // not used for this client
                6 => {
                    for _ in 0..conn.read_byte()? {
                        for _ in 0..4 {
                            let _ = conn.read_u16()?;
                        }
                    }
                },
                7 => {
                    let w = conn.read_size()?;
                    let h = conn.read_size()?;
                    if conn.matches_version(ProtVer(0, 0, 1)) {
                        let _ = conn.read_byte()?;
                    }
                    let mut scores = [0u32; 6];
                    for i in 0..6 {
                        scores[i] = conn.read_score()?;
                    }
                    let mut tiles = byte_buf((w as usize)*(h as usize)*2);
                    for i in 0..(tiles.len()/2) {
                        let sp = conn.read_bsp()?;
                        tiles[i*2] = sp.0;
                        tiles[i*2+1] = sp.1;
                    }
                    win2.emit("full-board", BoardInfo{w,h,tiles,scores})?;
                    gamestate = GameState::Midgame;
                },
                x => {println!("RUNLOOP INVALID PRELOOP COMMCODE: {}", x);break;}
            }
        } else if gamestate == GameState::Midgame {
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
                    win2.emit("get-input", "")?;
                    while !*hasinput {
                        hasinput = cvar.wait(hasinput).unwrap();
                    }
                    let pm = (*pmlock.lock().unwrap()).clone();
                    if conn.try_move(pm.0, pm.1)? {
                        break 'outer;
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
                    *go = is_true(event.payload().unwrap());
                    cvar.notify_one();
                });
                let (movex, movey) = conn.read_move()?;
                let team = conn.read_team()?;
                win2.emit("player-move", MoveInfo{x:movex,y:movey,team})?;
                while !*complete {
                    complete = cvar.wait(complete).unwrap();
                }
                if *gameover.lock().unwrap() {
                    gamestate = GameState::Pregame;
                }
            }
        }
    }
    Ok(())
}

pub fn entry(app: &mut tauri::App) -> TResult<()> {
    let handle = app.handle();
    let hand2a = app.handle();
    thread::spawn(move || {
        let window = handle.get_window("main").unwrap();
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
        window.listen("join-server", move |event|{
            let hand2 = hand2a.app_handle();
            let win2 = hand2.get_window("main").unwrap();
            // let sdat: usize = event.payload().unwrap().parse().unwrap();
            thread::spawn(move||{
                let jdat: JoinData = serde_json::from_str(event.payload().unwrap()).unwrap();
                match connect_server(&hand2, &jdat.cdat, jdat.name.clone()) {
                    Ok(s) => {
                        let _ = runloop(&hand2, s);
                    },
                    Err(e) => {
                        win2.emit("join-server-result", JoinResultPayload{ok:false, msg:format!("{}", e)}).unwrap();
                    }
                };
            });
        });
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