use crate::db::postgres_init;
use serde::{Deserialize, Serialize};
use crate::app::PartText;

#[derive(Serialize, Deserialize)]
pub struct Part {
    pub part_number: String,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
    pub label: Option<String>,
    pub package: Option<String>,
    pub value: Option<String>,
    pub tolerance: Option<String>,
}

fn new_part_from_sql(row: postgres::Row) -> Part {
    Part {
        part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
        manufacturer: Some(row.try_get("manufacturer").unwrap_or("".to_string())),
        description: Some(row.try_get("description").unwrap_or("".to_string())),
        label: Some(row.try_get("label").unwrap_or("".to_string())),
        package: Some(row.try_get("package").unwrap_or("".to_string())),
        value: Some(row.try_get("value").unwrap_or("".to_string())),
        tolerance: Some(row.try_get("tolerance").unwrap_or("".to_string()))
    }
}

pub fn get_mfg(pn: &str) -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT manufacturer from parts WHERE partnumber = $1", &[&pn]).unwrap();
    let _ = client.close();
    return row.get("manufacturer");
}
pub fn fetch_part_data() -> Vec<Part> {
    let mut client = postgres_init();
    let rows = client.query("SELECT * FROM parts", &[]).unwrap();
    let mut parts: Vec<Part> = Vec::new();
    for row in rows {
        let part = new_part_from_sql(row);
        parts.push(part);
    };
    return parts;
}

pub fn add_new_part(new_part: &PartText) {
    if(new_part.part_number == "".to_string()) {
        return;
    }
    let mut client = postgres_init();
    client.execute("INSERT INTO parts (partnumber, manufacturer, label, package, value, tolerance) VALUES ($1, $2, $3, $4, $5, $6)",
                   &[
                       &new_part.part_number,
                       &new_part.manufacturer,
                       &new_part.package,
                       &new_part.label,
                       &new_part.value,
                       &new_part.tolerance
                   ],
    ).unwrap();
}

pub fn retrieve_part(pn: &str) -> Part {
    let mut client = postgres_init();
    let row = client.query_one("SELECT * FROM parts WHERE partnumber = $1", &[&pn]).unwrap();
    let part = new_part_from_sql(row);
    let _ = client.close();
    return part;
}
pub fn modify_part(inpart: &Part) {
    if inpart.part_number == "".to_string() {
        println!("Part number cannot be empty!");
        return;
    }
    let mut client = postgres_init();
    client.execute("UPDATE parts SET manufacturer = $1, description = $2, label = $3, package = $4, value = $5, tolerance = $6 WHERE partnumber = $7",
                   &[
                       &inpart.manufacturer,
                       &inpart.description,
                       &inpart.label,
                       &inpart.package,
                       &inpart.value,
                       &inpart.tolerance,
                       &inpart.part_number
                   ],
    ).unwrap();
    let _ = client.close();
    return;
}