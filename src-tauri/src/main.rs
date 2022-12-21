#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use postgres::{Client, NoTls, Error};
use postgres::types::FromSql;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Part {
    part_number: String,
    manufacturer: String,
    description: String,
    label: String,
    package: String,
    value: String,
    tolerance: f64
}

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

#[tauri::command]
fn fetch_part_data() -> String {
    let mut client = postgres_init();
    let rows = client.query("SELECT * FROM parts", &[]).unwrap();
    let mut parts: Vec<Part> = Vec::new();
    for row in rows {
        let mut part = Part {
            part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
            manufacturer: row.try_get("manufacturer").unwrap_or("".to_string()),
            description: row.try_get("description").unwrap_or("".to_string()),
            label: row.try_get("label").unwrap_or("".to_string()),
            package: row.try_get("package").unwrap_or("".to_string()),
            value: row.try_get("value").unwrap_or("".to_string()),
            tolerance: row.try_get("tolerance").unwrap_or(0.0)
        };
        parts.push(part);
    };
    return serde_json::to_string(&parts).unwrap();
}

fn postgres_init() -> Client {
    return Client::connect("host=lapras.dex user=rootben password=password dbname=nudb", NoTls).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_mfg, get_pn, fetch_part_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
