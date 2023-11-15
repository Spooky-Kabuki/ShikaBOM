#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod parts;
use parts::{fetch_part_data, get_mfg, add_new_part, retrieve_part, modify_part};

mod db;
mod storage;
use storage::{fetch_storage_data};

use tauri::{command};

//Debugging tool to call from Svelte to print string to console
#[command]
fn print_to_console(s: &str) {
    println!("Svelte Console: {}", s);
    return;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![print_to_console, get_mfg, fetch_part_data, add_new_part, retrieve_part, modify_part, fetch_storage_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
