pub mod app;
pub mod tabs;

use std::{collections::HashMap, io::stdout, process::exit};

use clap::{Parser, Subcommand, command};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode}}, prelude::*, widgets::{self, ListState}
};

use crate::{app::App};

fn main() -> Result<()> {
    simple_logging::log_to_file("aaa", log::LevelFilter::Info)?;
    color_eyre::install()?;
    std::panic::set_hook(Box::new(|panic| {
        execute!(stdout(), DisableMouseCapture).unwrap_or_else(|e| eprintln!("failed to disable mouse capture:\n{}", e));
        ratatui::restore();
        eprintln!("{}", panic);
    }));
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(terminal);
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    result
}
