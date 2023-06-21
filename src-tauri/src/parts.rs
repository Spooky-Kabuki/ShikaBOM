use crate::db::postgres_init;

use tauri::{command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Part {
    part_number: String,
    manufacturer: Option<String>,
    description: Option<String>,
    label: Option<String>,
    package: Option<String>,
    value: Option<String>,
    tolerance: Option<String>,
}

#[command]
pub fn get_mfg(pn: &str) -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT manufacturer from parts WHERE partnumber = $1", &[&pn]).unwrap();
    client.close();
    return row.get("manufacturer");
}

#[command]
pub fn fetch_part_data() -> String {
    let mut client = postgres_init();
    let rows = client.query("SELECT * FROM parts", &[]).unwrap();
    let mut parts: Vec<Part> = Vec::new();
    for row in rows {
        let part = Part {
            part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
            manufacturer: Some(row.try_get("manufacturer").unwrap_or("".to_string())),
            description: Some(row.try_get("description").unwrap_or("".to_string())),
            label: Some(row.try_get("label").unwrap_or("".to_string())),
            package: Some(row.try_get("package").unwrap_or("".to_string())),
            value: Some(row.try_get("value").unwrap_or("".to_string())),
            tolerance: Some(row.try_get("tolerance").unwrap_or("".to_string()))
        };
        parts.push(part);
    };
    return serde_json::to_string(&parts).unwrap();
}

#[command]
pub fn add_new_part(inpart: &str) {
    let parsed_part: Part = serde_json::from_str(&inpart).unwrap();
    if parsed_part.part_number == "".to_string() {
        println!("Part number cannot be empty!");
        return;
    }
    let mut client = postgres_init();
    client.execute("INSERT INTO parts (partnumber, manufacturer, description, label, package, value, tolerance) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    &[
        &parsed_part.part_number,
        &parsed_part.manufacturer,
        &parsed_part.description,
        &parsed_part.label,
        &parsed_part.package,
        &parsed_part.value,
        &parsed_part.tolerance
        ],
    ).unwrap();
    client.close();
    return;
}

#[command]
pub fn retrieve_part(pn: &str) -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT * FROM parts WHERE partnumber = $1", &[&pn]).unwrap();
    let part = Part {
        part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
        manufacturer: Some(row.try_get("manufacturer").unwrap_or("".to_string())),
        description: Some(row.try_get("description").unwrap_or("".to_string())),
        label: Some(row.try_get("label").unwrap_or("".to_string())),
        package: Some(row.try_get("package").unwrap_or("".to_string())),
        value: Some(row.try_get("value").unwrap_or("".to_string())),
        tolerance: Some(row.try_get("tolerance").unwrap_or("".to_string()))
    };
    client.close();
    return serde_json::to_string(&part).unwrap();
}

#[command]
pub fn modify_part(inpart: &str) {
    let parsed_part: Part = serde_json::from_str(&inpart).unwrap();
    if parsed_part.part_number == "".to_string() {
        println!("Part number cannot be empty!");
        return;
    }
    let mut client = postgres_init();
    client.execute("UPDATE parts SET manufacturer = $1, description = $2, label = $3, package = $4, value = $5, tolerance = $6 WHERE partnumber = $7",
    &[
        &parsed_part.manufacturer,
        &parsed_part.description,
        &parsed_part.label,
        &parsed_part.package,
        &parsed_part.value,
        &parsed_part.tolerance,
        &parsed_part.part_number
        ],
    ).unwrap();
    client.close();
    return;
}