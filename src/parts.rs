use crate::db::postgres_init;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Part {
    pub part_number: String,
    pub total_qty: Option<i64>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
    pub label: Option<String>,
    pub package: Option<String>,
    pub value: Option<String>,
    pub tolerance: Option<String>,
}

pub struct PartStorage {
    pub part_number: String,
    pub location: String,
    pub quantity: i64
}

fn new_part_from_sql(row: postgres::Row) -> Part {
    let new_part = Part {
        part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
        total_qty: Some(row.try_get("total_qty").unwrap_or(0)),
        manufacturer: Some(row.try_get("manufacturer").unwrap_or("".to_string())),
        description: Some(row.try_get("description").unwrap_or("".to_string())),
        label: Some(row.try_get("label").unwrap_or("".to_string())),
        package: Some(row.try_get("package").unwrap_or("".to_string())),
        value: Some(row.try_get("value").unwrap_or("".to_string())),
        tolerance: Some(row.try_get("tolerance").unwrap_or("".to_string()))
    };
    return new_part;
}

pub fn fetch_all_parts() -> Vec<Part> {
    let mut client = postgres_init();
    let rows = client.query("select * from big_part_view", &[]).unwrap();
    let mut parts: Vec<Part> = Vec::new();
    for row in rows {
        let part = new_part_from_sql(row);
        parts.push(part);
    };
    return parts;
}

pub fn fetch_single_part(pn: &str) -> Part {
    let mut client = postgres_init();
    let row = client.query_one("select * from big_part_view where partnumber = $1", &[&pn]).unwrap();
    let part = new_part_from_sql(row);
    return part;
}

//Does not include quantity, this is just part information
pub fn add_new_part(new_part: &Part) {
    if new_part.part_number == "".to_string() {
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
    //TODO: Add initial quantity to part_storage
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

pub fn fetch_part_storage_data(pn: &str) -> Vec<PartStorage> {
    let mut client = postgres_init();
    let rows = client.query("select * from part_storage where partnumber = $1", &[&pn]).unwrap();
    let mut part_stores: Vec<PartStorage> = Vec::new();
    for row in rows {
        let part = PartStorage {
            part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
            location: row.try_get("storage_loc_name").unwrap_or("".to_string()),
            quantity: row.try_get("quantity").unwrap_or(0)
        };
        part_stores.push(part);
    };
    return part_stores;

}

#[test]
fn test_fetch_all_parts() {
    let parts = fetch_all_parts();
    assert!(!parts.is_empty());
}

#[test]
fn test_fetch_single_part() {
    let part = fetch_single_part("HFW1V2210H4R7K");
    assert_eq!(part.part_number, "HFW1V2210H4R7K");
    assert!(part.total_qty.unwrap() > 0);

}
