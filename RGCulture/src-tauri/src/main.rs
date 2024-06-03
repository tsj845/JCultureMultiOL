// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod comm;
mod rt;
use std::env::args;

use comm::entry;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn get_is_debug() -> bool {
    args().any(|e|{e=="--dbg-ui"})
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![get_is_debug])
    .setup(|app| {
        let h = app.handle();
        app.get_window("main").unwrap().listen("close", move |_|{h.exit(0)});
        if !args().any(|e|{e == "--no-server"}) {
            return entry(app);
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
