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
use crate::projects_view::{ProjectSubState, ProjectsView};
use crate::stock_view::*;
use crate::ui::ui;

pub enum CurrentScreen {
    PartScreen,
    ProjectScreen,
    StockScreen
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub parts_view: PartsView,
    pub stock_view: StockView,
    pub projects_view: ProjectsView,
    pub exit: bool,
}
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::PartScreen,
            parts_view: PartsView::new(),
            stock_view: StockView::new(),
            projects_view: ProjectsView::new(),
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
            CurrentScreen::StockScreen => {
                self.handle_storage_keys(key_event);
            }
            CurrentScreen::ProjectScreen => {
                self.handle_project_keys(key_event);
            }
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

    fn handle_storage_keys(&mut self, key_event: KeyEvent) {
        match self.stock_view.stock_sub_state {
            StockSubState::StockMain => {
                if !self.handle_global_keys(key_event) {
                    self.stock_view.handle_main_keys(key_event.code);
                }
            },
            StockSubState::CreateStock => {
                self.stock_view.handle_create_stock_keys(key_event.code);
            },
            StockSubState::AddStock => {
                self.stock_view.handle_add_stock_keys(key_event.code);
            },
            StockSubState::SpendStock => {
                self.stock_view.handle_spend_stock_keys(key_event.code);
            },
            StockSubState::EditStock => {
                self.stock_view.handle_edit_stock_keys(key_event.code);
            }
        }
    }

    fn handle_project_keys(&mut self, key_event: KeyEvent) {
        match self.projects_view.sub_state {
            ProjectSubState::Main => {
                if !self.handle_global_keys(key_event) {
                    self.projects_view.handle_main_keys(key_event.code);
                }
            }
            ProjectSubState::ListMode => {
                self.projects_view.handle_list_mode_keys(key_event.code);
            }
            ProjectSubState::BOMMode => {
                self.projects_view.handle_bom_mode_keys(key_event.code);
            }
            ProjectSubState::CreateNewProject => {
                self.projects_view.handle_create_project_keys(key_event.code);
            }
            ProjectSubState::AddToBOM => {
                self.projects_view.handle_add_to_bom_keys(key_event.code);
            }
        }
    }

    // handles global key events when we don't want to override (e.g. quit)
    pub fn handle_global_keys(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit();
                true
            },
            KeyCode::Char('S') => {
                self.stock_view.fetch_stock_data();
                self.current_screen = CurrentScreen::StockScreen;
                true
            },
            KeyCode::Char('A') => {
                self.current_screen = CurrentScreen::PartScreen;
                true
            },
            KeyCode::Char('P') => {
                self.current_screen = CurrentScreen::ProjectScreen;
                true
            }
            _ => {false}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}