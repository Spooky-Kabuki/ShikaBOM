use color_eyre::eyre::Context;
use color_eyre::Result;
use ratatui::{
    backend::{Backend},
    Terminal,
};
use crossterm::{
    event::{
        self, Event, KeyCode,
        KeyEventKind,
    }
};
use crossterm::event::KeyEvent;
use ratatui::widgets::TableState;
use crate::parts;
use crate::parts::Part;

use crate::ui::ui;

pub enum CurrentScreen {
    PartScreen
}

pub enum PartsSubState {
    Main,
    NewPart,
    EditPart,
}

pub struct PartScrollInfo {
    pub scroll_position: usize,
    pub scroll_length: u16
}

impl PartScrollInfo {
    fn reset(&mut self) {
        self.scroll_position = 0;
        self.scroll_length = 0;
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub parts_sub_state: PartsSubState,
    pub currently_editing_part: CurrentlyEditingPart,
    pub part_text: PartText,
    pub part_table_state: TableState,
    pub part_data: Vec<Part>,
    pub show_details: bool,
    pub exit: bool,
    pub part_scroll_info: PartScrollInfo
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
    fn clear(&mut self) {
        self.part_number.clear();
        self.total_qty.clear();
        self.manufacturer.clear();
        self.package.clear();
        self.label.clear();
        self.value.clear();
        self.tolerance.clear();
        self.description.clear();
    }

    fn copy_from_db_part(&mut self, part: &Part) {
        self.part_number = part.part_number.clone();
        self.total_qty = part.total_qty.unwrap_or(0).to_string();
        self.manufacturer = part.manufacturer.clone().unwrap_or("".to_string());
        self.package = part.package.clone().unwrap_or("".to_string());
        self.label = part.label.clone().unwrap_or("".to_string());
        self.value = part.value.clone().unwrap_or("".to_string());
        self.tolerance = part.tolerance.clone().unwrap_or("".to_string());
        self.description = part.description.clone().unwrap_or("".to_string());
    }

    fn copy_to_db_part(&self, part: &mut Part) {
        part.part_number = self.part_number.clone();
        part.total_qty = Some(self.total_qty.parse().unwrap()); //TODO: evaluate keeping this a number?
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
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::PartScreen,
            parts_sub_state: PartsSubState::Main,
            currently_editing_part: CurrentlyEditingPart::PartNumber,
            part_text: PartText {
                part_number: "".to_string(),
                total_qty: "0".to_string(),
                manufacturer: "".to_string(),
                package: "".to_string(),
                label: "".to_string(),
                value: "".to_string(),
                tolerance: "".to_string(),
                description: "".to_string()
            },
            part_table_state: TableState::default(),
            part_data: Vec::new(),
            exit: false,
            show_details: false,
            part_scroll_info: PartScrollInfo {
                scroll_position: 0,
                scroll_length: 0
            }
        }
    }

    pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
        app.refresh_part_data();
        while !app.exit {
            terminal.draw(|f| ui(f, app))?;
            app.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).wrap_err_with(|| {
                    format!("handling key event failed:\n{key_event:#?}")
                })
            }
            _ => Ok(())
        }
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.current_screen {
            CurrentScreen::PartScreen => {
                self.handle_parts_keys(key_event);
            }
        }
        Ok(())
    }

    fn handle_parts_keys(&mut self, key_event: KeyEvent) {
        match self.parts_sub_state {
            PartsSubState::Main => {
                if !self.handle_global_keys(key_event) {
                    match key_event.code {
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
                }
            } //end of PartsSubState::Main
            PartsSubState::NewPart => {
                match key_event.code {
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
                        //TODO: fix!!!!
                        let mut new_part: Part = Part {
                            part_number: "".to_string(),
                            total_qty: None,
                            manufacturer: None,
                            description: None,
                            label: None,
                            package: None,
                            value: None,
                            tolerance: None,
                        };
                        self.part_text.copy_to_db_part(&mut new_part);
                        parts::add_new_part(&new_part);
                        self.parts_sub_state = PartsSubState::Main;
                        self.refresh_part_data();

                    },
                    _ => {}
                }
            } //end of PartsSubState::NewPart
            PartsSubState::EditPart => {
                match key_event.code {
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
                        let mut part = Part {
                            part_number: "".to_string(),
                            total_qty: None,
                            manufacturer: None,
                            description: None,
                            label: None,
                            package: None,
                            value: None,
                            tolerance: None,
                        };
                        self.part_text.copy_to_db_part(&mut part);
                        parts::modify_part(&part);
                        self.parts_sub_state = PartsSubState::Main;
                        self.refresh_part_data();
                    },
                    _ => {}
                }

            }
        }
    }

    // handles global key events when we don't want to override (e.g. quit)
    fn handle_global_keys(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit();
                true
            }
            _ => {false}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn refresh_part_data(&mut self) {
        self.part_data = parts::fetch_all_parts();
    }

    fn show_details(&mut self) {
        self.show_details = !self.show_details;
    }

    fn update_selected_part(&mut self) {
        match self.part_table_state.selected() {
            Some(selected) => {
                let selected_pn = self.part_data[selected].part_number.clone();
                let fetched_part = parts::fetch_single_part(&selected_pn);
                self.part_text.copy_from_db_part(&fetched_part);
            }
            None => {}
        }
    }
}