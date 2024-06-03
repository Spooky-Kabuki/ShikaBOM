use std::cmp::PartialEq;
use crossterm::event::KeyCode;
use ratatui::widgets::{ListState, TableState};
use crate::stock::*;


pub enum StockSubState {
    StockMain,
    CreateStock,
    AddStock,
    SpendStock,
    EditStock
}

pub enum CreateStockPartField {
    PartNumber,
    LowStockThreshold,
    OnHand,
    //TODO: include on_order and in_prod
}

pub struct CurrentlyEditingStock {
    pub partnumber: String,
    pub low_stock_threshold: String,
    pub on_hand: String,
    pub active_field: CreateStockPartField
}

impl CurrentlyEditingStock {
    pub fn new() -> CurrentlyEditingStock {
        CurrentlyEditingStock {
            partnumber: "".to_string(),
            low_stock_threshold: "".to_string(),
            on_hand: "".to_string(),
            active_field: CreateStockPartField::PartNumber
        }
    }

    pub fn copy_to_stock_info(&self) -> StockInfo {
        StockInfo {
            partnumber: self.partnumber.clone(),
            low_stock_threshold: self.low_stock_threshold.parse().unwrap_or(0),
            on_hand: self.on_hand.parse().unwrap_or(0),
            on_order: 0,
            in_prod: 0,
            total_stock: 0,
            balance: 0,
            available: 0
        }
    }
}
pub struct StockView {
    pub stock_sub_state: StockSubState,
    pub stock_data: Vec<StockInfo>,
    pub show_details: bool,
    pub stock_table_state: TableState,
    pub nonstocked_pns: Vec<String>,
    pub nonstocked_pn_list_state: ListState,
    pub currently_editing_stock: CurrentlyEditingStock
}

impl PartialEq for CreateStockPartField {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CreateStockPartField::PartNumber, CreateStockPartField::PartNumber) => true,
            (CreateStockPartField::LowStockThreshold, CreateStockPartField::LowStockThreshold) => true,
            (CreateStockPartField::OnHand, CreateStockPartField::OnHand) => true,
            _ => false
        }
    }
}

impl StockView {
    pub fn new() -> StockView {
        StockView {
            stock_sub_state: StockSubState::StockMain,
            stock_data: Vec::new(),
            show_details: false,
            stock_table_state: TableState::default(),
            nonstocked_pns: Vec::new(),
            nonstocked_pn_list_state: ListState::default(),
            currently_editing_stock: CurrentlyEditingStock::new()
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
            KeyCode::Char('c') => {
                self.nonstocked_pns = fetch_nonstocked_partnumbers();
                self.stock_sub_state = StockSubState::CreateStock;
            },
            KeyCode::Char('a') => {
                self.stock_sub_state = StockSubState::AddStock;
            },
            KeyCode::Char('s') => {
                self.stock_sub_state = StockSubState::SpendStock;
            },
            KeyCode::Char('e') => {
                self.stock_sub_state = StockSubState::EditStock;
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

    pub fn handle_create_stock_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.stock_sub_state = StockSubState::StockMain;
            },
            KeyCode::Down => {
                if self.currently_editing_stock.active_field == CreateStockPartField::PartNumber {
                    match self.nonstocked_pn_list_state.selected() {
                        Some(selected) => {
                            if selected < self.nonstocked_pns.len() - 1 {
                                self.nonstocked_pn_list_state.select(Some(selected + 1));
                            }
                        }
                        None => {
                            self.nonstocked_pn_list_state.select(Some(0));
                        }
                    }
                }
            },
            KeyCode::Up => {
                if self.currently_editing_stock.active_field == CreateStockPartField::PartNumber {
                    match self.nonstocked_pn_list_state.selected() {
                        Some(selected) => {
                            if selected > 0 {
                                self.nonstocked_pn_list_state.select(Some(selected - 1));
                            }
                        }
                        None => {
                            self.nonstocked_pn_list_state.select(Some(0));
                        }
                    }
                }
            },
            KeyCode::Tab => {
                match self.currently_editing_stock.active_field {
                    CreateStockPartField::PartNumber => {
                        self.currently_editing_stock.active_field = CreateStockPartField::LowStockThreshold;
                    },
                    CreateStockPartField::LowStockThreshold => {
                        self.currently_editing_stock.active_field = CreateStockPartField::OnHand;
                    },
                    CreateStockPartField::OnHand => {
                        self.currently_editing_stock.active_field = CreateStockPartField::PartNumber;
                    }
                }
            },
            KeyCode::Char(value) => {
                //Non-number values aren't allowed in this form
                if !value.is_ascii_digit() {
                    return;
                }
                match self.currently_editing_stock.active_field {
                    CreateStockPartField::LowStockThreshold => {
                        self.currently_editing_stock.low_stock_threshold.push(value);
                    },
                    CreateStockPartField::OnHand => {
                        self.currently_editing_stock.on_hand.push(value);
                    },
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                match self.currently_editing_stock.active_field {
                    CreateStockPartField::LowStockThreshold => {
                        self.currently_editing_stock.low_stock_threshold.pop();
                    },
                    CreateStockPartField::OnHand => {
                        self.currently_editing_stock.on_hand.pop();
                    },
                    _ => {}
                }
            },
            KeyCode::Enter => {
                match self.nonstocked_pn_list_state.selected() {
                    Some(selected) => {
                        self.currently_editing_stock.partnumber =
                            self.nonstocked_pns[selected].clone();
                    },
                    //Don't do DB operations if a PN isn't selected.
                    None => { return; }
                }
                let new_stock = self.currently_editing_stock.copy_to_stock_info();
                create_new_stock(new_stock);
                //Reload the table after creating a new item
                self.fetch_stock_data();
                self.stock_sub_state = StockSubState::StockMain;
            },
            _ => {}
        }
    }

    pub fn handle_add_stock_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.stock_sub_state = StockSubState::StockMain;
            },
            _ => {}
        }
    }

    pub fn handle_spend_stock_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.stock_sub_state = StockSubState::StockMain;
            },
            _ => {}
        }
    }

    pub fn handle_edit_stock_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.stock_sub_state = StockSubState::StockMain;
            },
            _ => {}
        }
    }
}