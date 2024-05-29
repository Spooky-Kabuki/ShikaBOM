pub struct StockInfo {
    //Nothing in this struct can be null, so no optional types needed.
    partnumber: String,
    low_stock_threshold: i32,
    on_hand: i32,
    on_order: i32,
    in_prod: i32,
    total_stock: i32,
    balance: i32,
    available: i32
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