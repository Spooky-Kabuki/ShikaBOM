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

use crate::ui::ui;

pub enum CurrentScreen {
    Parts
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Parts,
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}