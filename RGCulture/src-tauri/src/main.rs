// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod comm;
mod rt;
use comm::entry;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .setup(|app| {
        entry(app)
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
