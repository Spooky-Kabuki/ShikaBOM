#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use postgres::{Client, NoTls};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_mfg(pn: &str) -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT manufacturer from parts WHERE partnumber = $1", &[&pn]).unwrap();
    client.close();
    return row.get("manufacturer");

}

#[tauri::command]
fn get_pn() -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT partnumber from parts WHERE partnumber = '1623022-1'", &[]).unwrap();
    client.close();
    return row.get("partnumber");
}

fn postgres_init() -> Client {
    return Client::connect("host=lapras.dex user=rootben password=password dbname=nudb", NoTls).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_mfg, get_pn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
