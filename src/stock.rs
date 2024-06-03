use crate::db;
pub struct StockInfo {
    //Nothing in this struct can be null, so no optional types needed.
    pub partnumber: String,
    pub low_stock_threshold: i32,
    pub on_hand: i32,
    pub on_order: i32,
    pub in_prod: i32,
    pub total_stock: i32,
    pub balance: i32,
    pub available: i32
}

impl StockInfo {
    pub fn new() -> StockInfo {
        StockInfo {
            partnumber: "".to_string(),
            low_stock_threshold: 0,
            on_hand: 0,
            on_order: 0,
            in_prod: 0,
            total_stock: 0,
            balance: 0,
            available: 0
        }
    }
}

pub fn fetch_stock_info() -> Vec<StockInfo> {
    let mut stock_data = Vec::new();

    let query = "SELECT * FROM stock";
    let mut client = db::postgres_init();
    let rows = client.query(query, &[]).unwrap();
    for row in rows {
        let mut stock = StockInfo::new();
        stock.partnumber = row.try_get("partnumber").unwrap_or("".to_string());
        stock.low_stock_threshold = row.try_get("low_stock_threshold").unwrap_or(0);
        stock.on_hand = row.try_get("on_hand").unwrap_or(0);
        stock.on_order = row.try_get("on_order").unwrap_or(0);
        stock.in_prod = row.try_get("in_prod").unwrap_or(0);
        stock.total_stock = row.try_get("c_stock").unwrap_or(0);
        stock.balance = row.try_get("c_balance").unwrap_or(0);
        stock.available = row.try_get("c_available").unwrap_or(0);
        stock_data.push(stock);
    }
    return stock_data;
}

pub fn fetch_nonstocked_partnumbers() -> Vec<String> {
    let mut partnumbers = Vec::new();
    let query = "select * from non_stocked_parts_view";
    let mut client = db::postgres_init();
    let rows = client.query(query, &[]).unwrap_or(Vec::new());
    for row in rows {
        let partnumber: String = row.try_get("partnumber").unwrap_or("".to_string());
        partnumbers.push(partnumber);
    }
    partnumbers
}

pub fn create_new_stock(stock: StockInfo) {
    let query = "INSERT INTO stock (partnumber, low_stock_threshold, on_hand, on_order) VALUES ($1, $2, $3, $4)";
    let mut client = db::postgres_init();
    client.execute(query, &[&stock.partnumber, &stock.low_stock_threshold, &stock.on_hand, &stock.on_order]).unwrap();
}