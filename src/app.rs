use std::{error::Error, io};
use color_eyre::eyre::Context;
use color_eyre::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
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

pub struct App {
    pub current_screen: CurrentScreen,
    pub parts_sub_state: PartsSubState,
    pub currently_editing_part: CurrentlyEditingPart,
    pub part_text: PartText,
    pub part_table_state: TableState,
    pub part_data: Vec<Part>,
    pub exit: bool,
}
pub struct PartText {
    pub part_number: String,
    pub manufacturer: String,
    pub package: String,
    pub label: String,
    pub value: String,
    pub tolerance: String,
}

impl PartText {
    fn clear(&mut self) {
        self.part_number.clear();
        self.manufacturer.clear();
        self.package.clear();
        self.label.clear();
        self.value.clear();
        self.tolerance.clear();
    }

    fn copy_from_db_part(&mut self, part: &Part) {
        self.part_number = part.part_number.clone();
        self.manufacturer = part.manufacturer.clone().unwrap_or("".to_string());
        self.package = part.package.clone().unwrap_or("".to_string());
        self.label = part.label.clone().unwrap_or("".to_string());
        self.value = part.value.clone().unwrap_or("".to_string());
        self.tolerance = part.tolerance.clone().unwrap_or("".to_string());
    }

    fn copy_to_db_part(&self, part: &mut Part) {
        part.part_number = self.part_number.clone();
        part.manufacturer = Some(self.manufacturer.clone());
        part.package = Some(self.package.clone());
        part.label = Some(self.label.clone());
        part.value = Some(self.value.clone());
        part.tolerance = Some(self.tolerance.clone());
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
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::PartScreen,
            parts_sub_state: PartsSubState::Main,
            currently_editing_part: CurrentlyEditingPart::PartNumber,
            part_text: PartText {
                part_number: "".to_string(),
                manufacturer: "".to_string(),
                package: "".to_string(),
                label: "".to_string(),
                value: "".to_string(),
                tolerance: "".to_string(),
            },
            part_table_state: TableState::default(),
            part_data: Vec::new(),
            exit: false
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
                if(!self.handle_global_keys(key_event)) {
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
                                    let fetched_part = parts::retrieve_part(&selected_pn);
                                    self.part_text.copy_from_db_part(&fetched_part);

                                    self.parts_sub_state = PartsSubState::EditPart;
                                    self.currently_editing_part = CurrentlyEditingPart::PartNumber;
                                }
                                None => {}
                            }


                        }
                        KeyCode::Down => {
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
                        }
                        KeyCode::Up => {
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
                                self.currently_editing_part = CurrentlyEditingPart::PartNumber;
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
                        parts::add_new_part(&self.part_text);
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
                                self.currently_editing_part = CurrentlyEditingPart::PartNumber;
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
                        //update the part in SQL
                        let mut part = Part {
                            part_number: "".to_string(),
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
        self.part_data = parts::fetch_part_data();
    }
}