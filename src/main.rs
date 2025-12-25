pub mod app;
pub mod tabs;
pub mod template_info;

use std::{collections::HashMap, io::stdout, process::exit};

use clap::{Parser, Subcommand, command};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode}}, prelude::*, widgets::{self, ListState}
};

use crate::{app::App, template_info::fetch_template_info};

fn main() -> Result<()> {
    simple_logging::log_to_file(".logs", log::LevelFilter::Info)?;
    color_eyre::install()?;
    let eyre_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        eyre_hook(panic_info);
        eprintln!("{}", panic_info);
    }));
    let terminal = ratatui::init();
    fetch_template_info().unwrap_or_else(|err| panic!("{err}"));
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}
