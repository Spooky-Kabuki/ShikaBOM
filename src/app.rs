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
use crate::app::CurrentScreen::Parts;
use crate::parts;

use crate::ui::ui;

pub enum CurrentScreen {
    Parts
}

pub enum PartsSubState {
    Main,
    NewPart
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub parts_sub_state: PartsSubState,
    pub currently_editing_part: CurrentlyEditingPart,
    pub part_text: PartText,
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
            current_screen: CurrentScreen::Parts,
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
            exit: false
        }
    }

    pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
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
            CurrentScreen::Parts => {
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
                        _ => {}
                    }
                }
            } //end of PartsSubState::Main
            PartsSubState::NewPart => {
                match key_event.code {
                    KeyCode::Esc => {
                        self.parts_sub_state = PartsSubState::Main;
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
                        parts::add_new_part_rat(&self.part_text);
                        self.parts_sub_state = PartsSubState::Main;

                    },
                    _ => {}
                }
            } //end of PartsSubState::NewPart
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
}