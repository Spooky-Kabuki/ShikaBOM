use crossterm::event::KeyCode;
use ratatui::widgets::TableState;
use crate::stock::*;


pub enum StockSubState {
    StockMain,
}
pub struct StockView {
    pub stock_sub_state: StockSubState,
    pub stock_data: Vec<StockInfo>,
    pub show_details: bool,
    pub stock_table_state: TableState,
}

impl StockView {
    pub fn new() -> StockView {
        StockView {
            stock_sub_state: StockSubState::StockMain,
            stock_data: Vec::new(),
            show_details: false,
            stock_table_state: TableState::default(),
        }
    }

    pub fn fetch_stock_data(&mut self) {
        self.stock_data = fetch_stock_info();
    }

    pub fn handle_main_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('d') => {
                self.show_details = !self.show_details;
            },
            KeyCode::Down => {
                match self.stock_table_state.selected() {
                    Some(selected) => {
                        if selected < self.stock_data.len() - 1 {
                            self.stock_table_state.select(Some(selected + 1));
                        }
                    }
                    None => {
                        self.stock_table_state.select(Some(0));
                    }
                }
            },
            KeyCode::Up => {
                match self.stock_table_state.selected() {
                    Some(selected) => {
                        if selected > 0 {
                            self.stock_table_state.select(Some(selected - 1));
                        }
                    }
                    None => {
                        self.stock_table_state.select(Some(0));
                    }
                }
            },
            _ => {}
        }
    }
}