mod tui;
mod errors;
mod app;
mod ui;
mod parts;
mod db;
mod parts_view;
mod parts_ui;
mod stock;
mod stock_ui;
mod stock_view;
mod projects;
mod projects_view;
mod projects_ui;
mod logging;
mod utils;

use app::App;

use color_eyre::{
    Result,
};

fn main() -> Result<()> {
    errors::install_hooks()?;
    logging::init()?;
    let mut terminal = tui::init()?;
    let mut app = App::new();
    App::run(&mut terminal, &mut app)?;
    tui::restore()?;
    Ok(())
}