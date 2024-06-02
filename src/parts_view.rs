use crossterm::event::KeyCode;
use ratatui::widgets::TableState;
use crate::parts;
use crate::parts::Part;

pub enum PartsSubState {
    Main,
    NewPart,
    EditPart,
}
impl PartialEq for PartsSubState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PartsSubState::Main, PartsSubState::Main) => true,
            (PartsSubState::NewPart, PartsSubState::NewPart) => true,
            (PartsSubState::EditPart, PartsSubState::EditPart) => true,
            _ => false,
        }
    }
}

pub struct PartScrollInfo {
    pub scroll_position: usize,
    pub scroll_length: u16
}

impl PartScrollInfo {
    pub(crate) fn reset(&mut self) {
        self.scroll_position = 0;
        self.scroll_length = 0;
    }
}

pub struct PartText {
    pub part_number: String,
    pub total_qty: String,
    pub manufacturer: String,
    pub package: String,
    pub label: String,
    pub value: String,
    pub tolerance: String,
    pub description: String,
}

impl PartText {
    pub(crate) fn clear(&mut self) {
        self.part_number.clear();
        self.total_qty.clear();
        self.manufacturer.clear();
        self.package.clear();
        self.label.clear();
        self.value.clear();
        self.tolerance.clear();
        self.description.clear();
    }

    pub(crate) fn copy_from_db_part(&mut self, part: &Part) {
        self.part_number = part.part_number.clone();
        self.total_qty = part.total_qty.unwrap_or(0).to_string();
        self.manufacturer = part.manufacturer.clone().unwrap_or("".to_string());
        self.package = part.package.clone().unwrap_or("".to_string());
        self.label = part.label.clone().unwrap_or("".to_string());
        self.value = part.value.clone().unwrap_or("".to_string());
        self.tolerance = part.tolerance.clone().unwrap_or("".to_string());
        self.description = part.description.clone().unwrap_or("".to_string());
    }

    pub(crate) fn copy_to_db_part(&self, part: &mut Part) {
        part.part_number = self.part_number.clone();
        part.total_qty = Some(self.total_qty.parse().unwrap_or(0)); //TODO: evaluate keeping this a number?
        part.manufacturer = Some(self.manufacturer.clone());
        part.package = Some(self.package.clone());
        part.label = Some(self.label.clone());
        part.value = Some(self.value.clone());
        part.tolerance = Some(self.tolerance.clone());
        part.description = Some(self.description.clone());
    }
}

pub enum CurrentlyEditingPart {
    PartNumber,
    Manufacturer,
    Package,
    Label,
    Value,
    Tolerance,
}

pub struct PartsView {
    pub parts_sub_state: PartsSubState,
    pub currently_editing_part: CurrentlyEditingPart,
    pub part_text: PartText,
    pub part_data: Vec<Part>,
    pub part_storage_data: Vec<parts::PartStorage>,
    pub show_details: bool,
    pub part_scroll_info: PartScrollInfo,
    //TODO: this might be better shared??? idk duplicate for now
    pub part_table_state: TableState,
}

impl PartsView {
    pub fn new () -> PartsView {
        PartsView {
            parts_sub_state: PartsSubState::Main,
            currently_editing_part: CurrentlyEditingPart::PartNumber,
            part_text: PartText {
                part_number: "".to_string(),
                total_qty: "".to_string(),
                manufacturer: "".to_string(),
                package: "".to_string(),
                label: "".to_string(),
                value: "".to_string(),
                tolerance: "".to_string(),
                description: "".to_string(),
            },
            part_data: Vec::new(),
            part_storage_data: Vec::new(),
            show_details: false,
            part_scroll_info: PartScrollInfo {
                scroll_position: 0,
                scroll_length: 0,
            },
            part_table_state: TableState::default(),
        }
    }
    pub fn refresh_part_data(&mut self) {
        self.part_data = parts::fetch_all_parts();
    }
    pub fn show_details(&mut self) {
        self.show_details = !self.show_details;
    }

    pub fn update_selected_part(&mut self) {
        match self.part_table_state.selected() {
            Some(selected) => {
                let selected_pn = self.part_data[selected].part_number.clone();
                let fetched_part = parts::fetch_single_part(&selected_pn);
                self.part_text.copy_from_db_part(&fetched_part);
            }
            None => {}
        }
    }

    pub fn handle_main_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('n') => {
                self.parts_sub_state = PartsSubState::NewPart;
                self.part_text.clear();
                self.currently_editing_part = CurrentlyEditingPart::PartNumber;
            }
            KeyCode::Char('r') => {
                self.refresh_part_data()
            }
            KeyCode::Char('e') => {
                match self.part_table_state.selected() {
                    Some(selected) => {
                        self.part_text.clear();
                        //Fill in part info
                        let selected_pn = self.part_data[selected].part_number.clone();
                        let fetched_part = parts::fetch_single_part(&selected_pn);
                        self.part_text.copy_from_db_part(&fetched_part);

                        self.parts_sub_state = PartsSubState::EditPart;
                        //Can't edit part number
                        self.currently_editing_part = CurrentlyEditingPart::Manufacturer;
                    }
                    None => {}
                }
            }
            KeyCode::Char('d') => {
                match self.part_table_state.selected() {
                    Some(selected) => {
                        self.part_text.clear();
                        //Fill in part info for side panel
                        //TODO: Make part text a big boi
                        let selected_pn = self.part_data[selected].part_number.clone();
                        let fetched_part = parts::fetch_single_part(&selected_pn);
                        self.part_text.copy_from_db_part(&fetched_part);
                        self.part_storage_data = parts::fetch_part_storage_data(&selected_pn);
                        self.part_scroll_info.reset();
                        //Only show if we have data to display
                        self.show_details();
                    }
                    None => {}
                }
            }
            KeyCode::Down => {
                if !self.show_details {
                    match self.part_table_state.selected() {
                        Some(selected) => {
                            if selected < self.part_data.len() - 1 {
                                self.part_table_state.select(Some(selected + 1));
                            }
                        }
                        None => {
                            self.part_table_state.select(Some(0));
                        }
                    }
                    self.update_selected_part();
                }
                else {
                    //TODO: scroll logic here
                    //TODO: how do we get scroll length?
                    self.part_scroll_info.scroll_position += 1;
                }
            }
            KeyCode::Up => {
                if !self.show_details {
                    match self.part_table_state.selected() {
                        Some(selected) => {
                            if selected > 0 {
                                self.part_table_state.select(Some(selected - 1));
                            }
                        }
                        None => {
                            self.part_table_state.select(Some(0));
                        }
                    }
                    self.update_selected_part();
                }
                else {
                    if self.part_scroll_info.scroll_position > 0 {
                        self.part_scroll_info.scroll_position -= 1;
                    }
                }

            }
            _ => {}
        }
    } //end handle_main_keys

    pub fn handle_new_part_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                //TODO: handle state transitions better
                self.parts_sub_state = PartsSubState::Main;
                self.refresh_part_data();

            }
            KeyCode::Char(value) => {
                match self.currently_editing_part {
                    CurrentlyEditingPart::PartNumber => {
                        self.part_text.part_number.push(value);
                    },
                    CurrentlyEditingPart::Manufacturer => {
                        self.part_text.manufacturer.push(value);
                    },
                    CurrentlyEditingPart::Package => {
                        self.part_text.package.push(value);
                    },
                    CurrentlyEditingPart::Label => {
                        self.part_text.label.push(value);
                    },
                    CurrentlyEditingPart::Value => {
                        self.part_text.value.push(value);
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.part_text.tolerance.push(value);
                    },
                }
            },
            KeyCode::Tab => {
                match self.currently_editing_part {
                    CurrentlyEditingPart::PartNumber => {
                        self.currently_editing_part = CurrentlyEditingPart::Manufacturer;
                    },
                    CurrentlyEditingPart::Manufacturer => {
                        self.currently_editing_part = CurrentlyEditingPart::Package;
                    },
                    CurrentlyEditingPart::Package => {
                        self.currently_editing_part = CurrentlyEditingPart::Label;
                    },
                    CurrentlyEditingPart::Label => {
                        self.currently_editing_part = CurrentlyEditingPart::Value;
                    },
                    CurrentlyEditingPart::Value => {
                        self.currently_editing_part = CurrentlyEditingPart::Tolerance;
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.currently_editing_part = CurrentlyEditingPart::Manufacturer;
                    },
                }
            },
            KeyCode::Backspace => {
                match self.currently_editing_part {
                    CurrentlyEditingPart::PartNumber => {
                        self.part_text.part_number.pop();
                    },
                    CurrentlyEditingPart::Manufacturer => {
                        self.part_text.manufacturer.pop();
                    },
                    CurrentlyEditingPart::Package => {
                        self.part_text.package.pop();
                    },
                    CurrentlyEditingPart::Label => {
                        self.part_text.label.pop();
                    },
                    CurrentlyEditingPart::Value => {
                        self.part_text.value.pop();
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.part_text.tolerance.pop();
                    },
                }
            },
            KeyCode::Enter => {
                //TODO: fix!!!! | now I'm not sure what to fix lol
                let mut new_part = Part::new();
                self.part_text.copy_to_db_part(&mut new_part);
                parts::add_new_part(&new_part);
                self.parts_sub_state = PartsSubState::Main;
                self.refresh_part_data();

            },
            _ => {}
        }
    } //end handle_new_keys
    
    pub fn handle_edit_part_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                //TODO: handle state transitions better
                self.parts_sub_state = PartsSubState::Main;
                self.refresh_part_data();
            }
            KeyCode::Char(value) => {
                match self.currently_editing_part {
                    CurrentlyEditingPart::Manufacturer => {
                        self.part_text.manufacturer.push(value);
                    },
                    CurrentlyEditingPart::Package => {
                        self.part_text.package.push(value);
                    },
                    CurrentlyEditingPart::Label => {
                        self.part_text.label.push(value);
                    },
                    CurrentlyEditingPart::Value => {
                        self.part_text.value.push(value);
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.part_text.tolerance.push(value);
                    },
                    _ => {}
                }
            },
            KeyCode::Tab => {
                match self.currently_editing_part {
                    //Shouldn't be in the PN state, but just in case
                    CurrentlyEditingPart::PartNumber => {
                        self.currently_editing_part = CurrentlyEditingPart::Manufacturer;
                    },
                    CurrentlyEditingPart::Manufacturer => {
                        self.currently_editing_part = CurrentlyEditingPart::Package;
                    },
                    CurrentlyEditingPart::Package => {
                        self.currently_editing_part = CurrentlyEditingPart::Label;
                    },
                    CurrentlyEditingPart::Label => {
                        self.currently_editing_part = CurrentlyEditingPart::Value;
                    },
                    CurrentlyEditingPart::Value => {
                        self.currently_editing_part = CurrentlyEditingPart::Tolerance;
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.currently_editing_part = CurrentlyEditingPart::Manufacturer;
                    },
                }
            },
            KeyCode::Backspace => {
                match self.currently_editing_part {
                    CurrentlyEditingPart::Manufacturer => {
                        self.part_text.manufacturer.pop();
                    },
                    CurrentlyEditingPart::Package => {
                        self.part_text.package.pop();
                    },
                    CurrentlyEditingPart::Label => {
                        self.part_text.label.pop();
                    },
                    CurrentlyEditingPart::Value => {
                        self.part_text.value.pop();
                    },
                    CurrentlyEditingPart::Tolerance => {
                        self.part_text.tolerance.pop();
                    },
                    _ => {}
                }
            },
            KeyCode::Enter => {
                //update the part in SQL
                let mut part = Part::new();
                self.part_text.copy_to_db_part(&mut part);
                parts::modify_part(&part);
                self.parts_sub_state = PartsSubState::Main;
                self.refresh_part_data();
            },
            _ => {}
        }

    } //end handle_edit_keys
}
