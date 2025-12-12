use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    process::exit,
};

use crate::{app::RenderCommands, tabs::Tab};
use log::info;
use ratatui::{
    crossterm::event::{Event, MouseButton, MouseEventKind},
    prelude::*,
    widgets::{self, Block, Borders, ListState},
};
use tui_input::{Input, backend::crossterm::EventHandler};
#[derive(Default)]
pub struct ScaffoldTab {
    focus: ScaffoldTabFocus,
    searchbar_input: Input,
    list_state: ListState,
    areas: ScaffoldTabAreas,
    list_data_search_query: String,
    list_data: Vec<ScaffoldListEntry>,
}

impl ScaffoldTab {
    fn update_list(&mut self) {
        let a  = vec!["a"];
        self.list_data_search_query.contains(self.searchbar_input.value());
    }
}

#[derive(Debug, Default)]
pub struct ScaffoldListEntry {}
#[derive(Debug, Default)]
pub struct ScaffoldTabAreas {
    searchbar: Rect,
    list: Rect,
}
impl<'a> From<&ScaffoldListEntry> for widgets::ListItem<'a> {
    fn from(value: &ScaffoldListEntry) -> Self {
        widgets::ListItem::new("a")
    }
}

#[derive(Debug, Default)]
pub enum ScaffoldTabFocus {
    #[default]
    Searchbar,
    List(usize),
}
impl Tab for ScaffoldTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer, commands: &mut RenderCommands) {
        let [searchbar_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        self.areas.list = list_area;
        self.areas.searchbar = searchbar_area;
        let mut searchbar_text = self.searchbar_input.value().to_string();
        if let ScaffoldTabFocus::Searchbar = self.focus {
            if searchbar_text.len() == self.searchbar_input.cursor() {
                searchbar_text.push('\u{2588}');
            } else {
                let cursor_pos = self.searchbar_input.cursor();
                searchbar_text.replace_range(cursor_pos..=cursor_pos, "\u{2588}");
            }
        }
        let mut searchbar = widgets::Paragraph::new(searchbar_text)
            .scroll((
                0,
                self.searchbar_input
                    .visual_scroll(searchbar_area.width as usize) as u16,
            ))
            .block(Block::bordered().title("Find Installed"));

        searchbar.render(searchbar_area, buf);

        let list = widgets::List::new(self.list_data.iter()).block(Block::new().title("abc"));
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);
    }
    fn handle_event(&mut self, ev: Event) {
        match &ev {
            Event::Mouse(mouse_ev) => {
                let mouse_pos = Position {
                    x: mouse_ev.column,
                    y: mouse_ev.row,
                };
                if let MouseEventKind::Down(MouseButton::Left) = mouse_ev.kind {
                    if self.areas.list.contains(mouse_pos) {
                        if let Some(list_entry) = self.list_data.first() {
                            let list_item_height = widgets::ListItem::from(list_entry).height();
                            self.focus =
                                ScaffoldTabFocus::List(self.list_state.offset() / list_item_height);
                        } else {
                            self.focus = ScaffoldTabFocus::List(0);
                        }
                    } else if self.areas.searchbar.contains(mouse_pos) {
                        self.focus = ScaffoldTabFocus::Searchbar;
                    }
                }
            }
            _ => {}
        }
        match self.focus {
            ScaffoldTabFocus::Searchbar => {
                self.searchbar_input.handle_event(&ev);
                if self.searchbar_input.value() != self.list_data_search_query {
                    self.list_data_search_query = self.searchbar_input.value().to_owned();
                    self.update_list();
                }
            }
            ScaffoldTabFocus::List(list_idx) => {}
        }
    }
}
