use crate::db::postgres_init;

use tauri::{command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StorageLoc {
    part_number: String,
    location: Option<String>,
    quantity: Option<i32>,
    loc_id: Option<String>
}

#[derive(Serialize, Deserialize)]
struct PartQty {
    part_number: String,
    quantity: Option<i32>
}

#[command]
pub fn fetch_storage_data() -> String {
    let mut client = postgres_init();
    let rows = client.query("SELECT * FROM part_storage INNER JOIN storage_locs p on p.storage_loc_id = part_storage.storage_loc_id", &[]).unwrap();
    let mut storage: Vec<StorageLoc> = Vec::new();
    for row in rows {
        let loc = StorageLoc {
            part_number: row.try_get("partnumber").unwrap_or("".to_string()), //this cannot be null
            location: Some(row.try_get("storage_loc_name").unwrap_or("".to_string())),
            quantity: Some(row.try_get("quantity").unwrap_or(0)),
            loc_id: Some(row.try_get("storage_loc_id").unwrap_or("".to_string()))
        };
        storage.push(loc);
    };
    return serde_json::to_string(&storage).unwrap();
}

#[command]
pub fn retrieve_qty(pn: &str) -> String {
    let mut client = postgres_init();
    let row = client.query_one("SELECT * from part_storage WHERE partnumber = $1", &[&pn]).unwrap();
    let _ = client.close();
    let qty = PartQty {
        part_number: row.try_get("partnumber").unwrap_or("bingus".to_string()),
        quantity: Some(row.try_get("quantity").unwrap_or(0))
    };
    return serde_json::to_string(&qty).unwrap();
}

#[command]
pub fn modify_qty(inpart: &str) {
    let parsed_part: PartQty = serde_json::from_str(&inpart).unwrap();
    if parsed_part.part_number == "".to_string() {
        println!("Part number cannot be empty!");
        return;
    }
    let existing_qty = get_qty(&parsed_part.part_number);
    let new_qty: i32 = (&parsed_part.quantity).unwrap_or(existing_qty);
    let mut client = postgres_init();
    client.execute("UPDATE part_storage SET quantity = $1 WHERE partnumber = $2",
    &[
        &new_qty,
        &parsed_part.part_number
    ]).unwrap();
    let _ = client.close();
    return;
}

fn get_qty(pn: &str) -> i32 {
    let mut client = postgres_init();
    let row = client.query_one("SELECT quantity from part_storage WHERE partnumber = $1", &[&pn]).unwrap();
    let _ = client.close();
    //TODO: this breaks if the part doesn't have a quantity
    return row.get("quantity");
}