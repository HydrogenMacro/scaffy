use std::{collections::HashMap, process::exit};

use clap::{Parser, Subcommand, command};
use color_eyre::Result;
use log::info;
use ratatui::{
    DefaultTerminal, crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, prelude::*, widgets::{self, ListState}
};

use crate::tabs::{Tab, scaffold::ScaffoldTab};

#[derive(Debug, Default)]
pub struct RenderCommands {
    set_cursor_pos: Option<Position>
}
impl RenderCommands {
    pub fn set_cursor_pos(&mut self, position: Position) {
        self.set_cursor_pos = Some(position);
    }
}

pub struct AppTab {
    label: &'static str,
    tab: Box<dyn Tab>,
}
pub struct App {
    current_tab_idx: usize,
    tabs: Vec<AppTab>,
}
impl App {
    pub fn new() -> Self {
        Self {
            current_tab_idx: 0,
            tabs: vec![
                AppTab {
                    label: "Scaffold",
                    tab: Box::new(ScaffoldTab::default()),
                },
                AppTab {
                    label: "Scaffold",
                    tab: Box::new(ScaffoldTab::default()),
                },
                AppTab {
                    label: "Scaffold",
                    tab: Box::new(ScaffoldTab::default()),
                },
            ],
        }
    }
    pub fn get_current_tab(&self) -> &dyn Tab {
        &*self.tabs[self.current_tab_idx].tab
    }
    pub fn get_current_tab_mut(&mut self) -> &mut dyn Tab {
        &mut *self.tabs[self.current_tab_idx].tab
    }
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            let mut commands = RenderCommands::default();
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut(), &mut commands))?;
            if let Some(new_cursor_pos) = commands.set_cursor_pos {
                info!("{:?}", new_cursor_pos);
                terminal.set_cursor_position(new_cursor_pos)?;
                info!("{:?}", terminal.get_cursor_position());
            }
            let ev = event::read()?;
            let mut handled = false;
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
            if !handled {
                self.get_current_tab_mut().handle_event(ev);
            }
        }
    }
    pub fn render(&mut self, area: Rect, buf: &mut Buffer, commands: &mut RenderCommands ) {
        let [tab_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        let tab_widget =
            widgets::Tabs::new(self.tabs.iter().map(|t| t.label)).select(self.current_tab_idx);

        tab_widget.render(tab_area, buf);
        self.get_current_tab_mut().render(content_area, buf, commands);
    }
}