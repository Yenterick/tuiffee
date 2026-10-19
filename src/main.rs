use color_eyre::eyre::Result;
use dotenvy::dotenv;

use crate::presentation::app;

mod config;
mod models;
mod presentation;
mod repositories;
mod services;

fn main() -> Result<()> {
    dotenv().ok();

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = app::run(terminal);
    ratatui::restore();

    result
}
