// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod payloads;
mod common;
mod comm;
mod rt;
mod protocol;
use std::{env::args, fs};
use std::path::Path;

use comm::entry;
use tauri::Manager;

#[tauri::command]
fn get_is_debug() -> bool {
    args().any(|e|{e=="--dbg-ui"})
}

#[tauri::command]
fn fetch_content(path: &str) -> String {
    let p = String::from("../ui/")+path;
    // println!("{p}");
    // println!("{}", fs::canonicalize(&p).unwrap().display());
    fs::read_to_string(p).unwrap()
}

const SERVER_LIST_PATH: &'static str = "../rgc_local_data/servers.txt";

#[tauri::command]
fn fetch_servers() -> String {
    if Path::exists(Path::new(SERVER_LIST_PATH)) {
        return fs::read_to_string(SERVER_LIST_PATH).unwrap();
    }
    return "".to_string();
}

#[tauri::command]
fn store_servers(servers: String) -> () {
    fs::write(SERVER_LIST_PATH, servers).unwrap();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_content, get_is_debug, fetch_servers, store_servers])
    .setup(|app| {
        let h = app.handle();
        let win = app.get_window("main").unwrap();
        win.listen("close", move |_|{h.exit(0)});
        win.listen("echo", |_|{println!("ECHO");});
        if !args().any(|e|{e == "--no-server"}) {
            return entry(app);
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
