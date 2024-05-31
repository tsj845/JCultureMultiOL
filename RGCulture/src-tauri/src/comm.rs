use crate::{common::*, rt::{Sockets, ErrCause}};
use tauri::Manager;
use std::{env::args, io::Write, thread};
use std::sync::mpsc::{Receiver, Sender, channel};
// use serde_json::Value as JSValue;
use std::sync::{Arc, Mutex, Condvar};

macro_rules! condvar {
    ($val:literal) => {
        Arc::new((Mutex::new($val), Condvar::new()))
    };
}

#[derive(Clone, Serialize, Deserialize)]
struct UIPlayerMovePayload {
    x: u32,
    y: u32
}

// struct Password {
//     pub value: String
// }

// impl Password {
//     fn new() -> Self {
//         Self{value:String::new()}
//     }
// }

pub fn entry(app: &mut tauri::App) -> TResult<()> {
    let window = app.get_window("main").unwrap();
    thread::spawn(move || {
    let (atx1, arx1): (Sender<String>, Receiver<String>) = channel();
    let (ttx1, trx1): (Sender<String>, Receiver<String>) = channel();
    let pwordcvar = condvar!(false);
    let pwordin = Arc::new(Mutex::new(String::new()));
    let pwin2 = pwordin.clone();
    let pwcv = pwordcvar.clone();
    {
        let thtx = atx1.clone();
        window.once("window-ready", move |_| {
            thtx.send(String::new()).unwrap();
        });
    }
    arx1.recv().unwrap();
    window.listen("input", |event| {
        let pmp: UIPlayerMovePayload = serde_json::from_str(event.payload().unwrap()).unwrap();
    });
    window.listen("password-input", move |event| {
        let (lock, cvar) = &*pwcv;
        let lock2 = &*pwin2;
        let mut l = lock.lock().unwrap();
        let pay = event.payload().unwrap();
        let pass = pay[1..pay.len()-1].to_string();
        println!("GOT PASSWORD: {}", &pass);
        if *l {
            println!("PASSWORD NOT READY");
            return;
        }
        let mut v = lock2.lock().unwrap();
        *v = pass;
        println!("PW LOADED");
        *l = true;
        cvar.notify_one();
        println!("PW SIGNALED");
    });
    println!("STARTED");
    {
        let thtx = atx1.clone();
        thread::spawn(move || {
            let cdata: ConnData;
            let cdtmsn = match ItcComm::from_string(trx1.recv().unwrap()) {
                ItcComm::Data(v) => v,
                _ => {return;}
            };
            cdata = serde_json::from_str(&cdtmsn).unwrap();
            match Sockets::connect_data(&cdata) {
                Ok(sd) => {
                    thtx.send(format!("DATA{}", serde_json::to_string(&sd).unwrap())).unwrap();
                },
                Err(_) => {
                    thtx.send("ERROR{\"msg\":\"connect_data_failure\"}".to_string()).unwrap();
                    return;
                }
            };
            let data: String = match ItcComm::from_string(trx1.recv().unwrap()) {
                ItcComm::Data(d) => d,
                _ => {return;}
            };
            if !(data == "true") {
                println!("USER REJECT SERVER");
                return;
            }
            println!("USER CONFIRM SERVER");
            let mut conn: Sockets = match Sockets::connect_player(&cdata) {
                Ok(c) => c,
                Err(_) => {thtx.send("ERROR{\"msg\":\"connect_player_failure\"}".to_string()).unwrap();return;}
            };
            println!("CONNECT OK");
            let hp: bool = match conn.check_password() {
                Ok(b)=>b,
                Err(c)=>{thtx.send(format!("ERROR{}", serde_json::to_string(&ItcError{msg:match c {
                    ErrCause::IO => "IO_ERROR".to_owned(),
                    ErrCause::SEQUENCE => "SEQUENCE_ERROR".to_owned()
                }}).unwrap())).unwrap();return;}
            };
            println!("PCHECK OK. PASSWORD={}", hp);
            thtx.send(format!("DATA{}", hp)).unwrap();
            if hp {
                // there is a password
                'outer: loop {
                    // get the password
                    let pw: String = trx1.recv().unwrap();
                    println!("GOT INPUT PASSWORD: {}", &pw);
                    // send the password
                    match conn.submit_password(&pw) {
                        // went ok
                        Ok(v) => {
                            println!("SUBMIT OK. ACCEPTED={}", v);
                            // send if pw was accepted
                            thtx.send(format!("DATA{}", v)).unwrap();match v {
                            // exit pw loop
                            true => {break 'outer;},
                            false => {
                                // get if user wants to try again
                                match ItcComm::from_string(trx1.recv().unwrap()) {
                                    ItcComm::Data(value) => {
                                        // get that value
                                        if value == "true" {
                                            println!("USER CONTINUE PW TRY");
                                            // check all went well in communicating to server
                                            match conn.decide_pw_continue(true) {
                                                Ok(_) => {println!("CONTINUE OK");thtx.send("OK".to_string()).unwrap();},
                                                Err(_) => {println!("PWCONT ERR");thtx.send("ERROR{\"msg\":\"pw_continue_failure\"}".to_string()).unwrap();return;}
                                            }
                                        } else {
                                            println!("USER PW GIVE UP");
                                            // don't care if there was an error here given that we're going to close the connection
                                            let _=conn.decide_pw_continue(false);
                                            return;
                                        }
                                    },
                                    _ => {return;}
                                }
                            }
                        }},
                        Err(_) => {conn.shutdown();thtx.send("ERROR{\"msg\":\"IO_ERROR\"}".to_string()).unwrap();return;}
                    };
                }
            }
            let name: String = match ItcComm::from_string(trx1.recv().unwrap()) {
                ItcComm::Data(v) => v,
                _ => {conn.shutdown();println!("NAME ERROR");return;}
            };
            println!("GOT NAME: {}", &name);
            match conn.send_name(name) {
                Ok(_) => {thtx.send("OK".to_string()).unwrap();},
                Err(_) => {conn.shutdown();thtx.send("ERROR{\"msg\":\"IO_ERROR\"}".to_string()).unwrap();return;}
            };
            match conn.get_team() {
                Ok(v) => {thtx.send(format!("DATA[{},{}]", v.0,v.1)).unwrap();},
                Err(_) => {conn.shutdown();thtx.send("ERROR{\"msg\":\"IO_ERROR\"}".to_string()).unwrap();return;}
            };
            let byte = 0u8;
            let _=conn.expose_comm().write_all(std::slice::from_ref(&byte));
            println!("NORMAL SHUTDOWN");
            let _=conn.shutdown();
        });
    }
    let cd = args().nth(1).unwrap();
    let cnd = ConnData::new(&cd[0..cd.find(':').unwrap()], (&cd[cd.find(':').unwrap()+1..]).parse().unwrap());
    ttx1.send(format!("DATA{}", serde_json::to_string(&cnd).unwrap())).unwrap();
    let res = ItcComm::from_string(arx1.recv().unwrap());
    println!("{:?}", res);
    ttx1.send("DATAtrue".to_string()).unwrap();
    match ItcComm::from_string(arx1.recv().unwrap()) {
        ItcComm::Data(v) => {
            if v == "true" {
                println!("PASSWORD YES");
                let (lock, cvar) = &*pwordcvar;
                let lock2 = &*pwordin;
                let mut hasinput = lock.lock().unwrap();
                *hasinput = false;
                println!("INPUT SETUP, PASSWORD PROMPTED");
                window.emit("prompt-password", "").unwrap();
                loop {
                    println!("PASSWORD WAIT");
                    while !*hasinput {
                        hasinput = cvar.wait(hasinput).unwrap();
                    }
                    let pw = (*lock2.lock().unwrap()).clone();
                    println!("PASSWORD WAIT END: {}", &pw);
                    ttx1.send(pw).unwrap();
                    match ItcComm::from_string(arx1.recv().unwrap()) {
                        ItcComm::Data(d) => {
                            println!("HOST PW CHECK DATA BACK: {}", &d);
                            if d == "true" {
                                println!("HOST PW ACCEPT");
                                window.emit("password-accept", "").unwrap();
                                break;
                            } else {
                                println!("HOST PW REJECT");
                                ttx1.send("DATAtrue".to_string()).unwrap();
                                match ItcComm::from_string(arx1.recv().unwrap()) {
                                    ItcComm::Data(_) => {println!("PW CONT OK");},
                                    _ => {println!("PW CONT ERR");return Err(GameError::boxed());}
                                };
                                println!("INPUT SETUP");
                                *hasinput = false;
                                window.emit("password-reject", "").unwrap();
                            }
                        },
                        _ => {println!("PW COMM FAIL");return Err(GameError::boxed());}
                    }
                }
            }
        },
        _ => {println!("BAD RESPONSE");return Err(GameError::boxed());}
    };
    ttx1.send("DATAtr1".to_string()).unwrap();
    println!("{}", arx1.recv().unwrap());
    println!("{}", arx1.recv().unwrap());
    Ok(())
    });
    Ok(())
}