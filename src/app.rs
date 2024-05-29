use color_eyre::eyre::Context;
use color_eyre::Result;
use ratatui::{
    backend::{Backend},
    Terminal,
};
use crossterm::{
    event::{
        self, Event, KeyCode, KeyEventKind, KeyEvent
    }
};
use crate::parts_view::*;
use crate::ui::ui;

pub enum CurrentScreen {
    PartScreen,
    ProjectScreen,
    StorageScreen
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub parts_view: PartsView,
    pub exit: bool,
}
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::PartScreen,
            parts_view: PartsView::new(),
            exit: false,
        }
    }

    pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
        app.parts_view.refresh_part_data();
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
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_parts_keys(&mut self, key_event: KeyEvent) {
        match self.parts_view.parts_sub_state {
            PartsSubState::Main => {
                if !self.handle_global_keys(key_event) {
                    self.parts_view.handle_main_keys(key_event.code);
                }
            } //end of PartsSubState::Main
            PartsSubState::NewPart => {
                self.parts_view.handle_new_part_keys(key_event.code);
            }
            PartsSubState::EditPart => {
                self.parts_view.handle_edit_part_keys(key_event.code);
            }
        }

    }

    // handles global key events when we don't want to override (e.g. quit)
    pub fn handle_global_keys(&mut self, key_event: KeyEvent) -> bool {
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