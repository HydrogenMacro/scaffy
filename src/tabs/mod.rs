pub mod project_init;
pub mod scaffold;
pub mod tag;

use clap::Subcommand;
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::Event,
    layout::{Position, Rect},
};

use crate::app::Commands;

pub trait Tab {
    fn handle_event(&mut self, ev: Event, commands: &mut Commands) {}
    fn render(&mut self, area: Rect, buf: &mut Buffer);
}
