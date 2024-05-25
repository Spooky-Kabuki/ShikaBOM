mod tui;
mod errors;
mod app;
mod ui;
mod parts;
mod db;

use app::App;

use color_eyre::{
    Result,
};

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let mut app = App::new();
    App::run(&mut terminal, &mut app)?;
    tui::restore()?;
    Ok(())
}