use std::mem;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    prelude::*,
};

use crate::tabs::{Tab, search::ScaffoldTab};

#[derive(Default)]
pub struct Commands {
    pub next_tab: Option<Box<dyn Tab>>,
    pub should_switch_tab_to_cached: bool,
    pub should_cache_current_tab: bool,
    pub should_quit: bool,
}
impl Commands {
    pub fn cache_current_tab(&mut self) {
        self.should_cache_current_tab = true;
    }
    pub fn switch_tab_to(&mut self, next_tab: impl Tab + 'static) {
        self.next_tab = Some(Box::new(next_tab));
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn switch_tab_to_cached(&mut self) {
        self.should_switch_tab_to_cached = true;
    }
}

pub struct App {
    current_tab: Box<dyn Tab>,
    cached_tab: Box<dyn Tab>,
}
impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Box::new(ScaffoldTab::new()),
            cached_tab: Box::new(ScaffoldTab::new()),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
            let ev = event::read()?;
            match ev {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }
                }
                _ => {}
            }
            let mut commands = Commands::default();
            self.current_tab.handle_event(ev, &mut commands);

            if commands.should_cache_current_tab {
                mem::swap(&mut self.cached_tab, &mut self.current_tab);
            }
            if let Some(next_tab) = commands.next_tab {
                self.current_tab = next_tab;
            }
            if commands.should_switch_tab_to_cached {
                mem::swap(&mut self.cached_tab, &mut self.current_tab);
            }
            if commands.should_quit {
                return Ok(());
            }
        }
    }
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        self.current_tab.render(area, buf);
    }
}
