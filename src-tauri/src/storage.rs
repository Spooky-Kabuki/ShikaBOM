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