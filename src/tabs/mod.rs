pub mod scaffold;

use clap::Subcommand;
use ratatui::{Frame, buffer::Buffer, crossterm::event::Event, layout::{Position, Rect}};

use crate::app::RenderCommands;

pub trait Tab {
    fn handle_event(&mut self, ev: Event) {}
    fn render(&mut self, area: Rect, buf: &mut Buffer, commands: &mut RenderCommands);
}
