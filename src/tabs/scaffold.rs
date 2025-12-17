use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    iter,
    process::{self, exit},
};

use crate::{
    app::Commands,
    tabs::{
        Tab, project_init::ProjectInitTab, tag::{Tag, TagType}
    },
};
use log::info;
use ratatui::{
    crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind},
    prelude::*,
    widgets::{self, Block, Borders, ListState, block::Title},
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
    pub fn new() -> Self {
        let mut scaffold_tab = ScaffoldTab::default();
        scaffold_tab.update_list();
        scaffold_tab
    }
    fn update_list(&mut self) {
        let a = vec![
            (
                "abc".to_owned(),
                vec![
                    Tag::new("abc".to_owned(), TagType::Framework, None),
                    Tag::new("abc".to_owned(), TagType::Language, None),
                    Tag::new("abc".to_owned(), TagType::Specialization, None),
                ],
            ),
            (
                "def".to_owned(),
                vec![Tag::new("abc".to_owned(), TagType::Framework, None)],
            ),
            (
                "ghi".to_owned(),
                vec![Tag::new("abc".to_owned(), TagType::Framework, None)],
            ),
            (
                "jkl".to_owned(),
                vec![Tag::new("abc".to_owned(), TagType::Framework, None)],
            ),
        ];
        self.list_data = a
            .into_iter()
            .map(|(name, tags)| ScaffoldListEntry::new(name, "template_id".into(), "author".into(), "description ".repeat(5), tags))
            .filter(|list_entry| list_entry.matches_query(&self.list_data_search_query))
            .collect();
        self.list_state.select(Some(0));
    }
}

#[derive(Debug, Default)]
pub struct ScaffoldListEntry {
    template_name: String,
    template_id: String,
    author: String,
    desc: String,
    tags: Vec<Tag>,
}
impl ScaffoldListEntry {
    pub fn new(template_name: String, template_id: String, author: String, desc: String, tags: Vec<Tag>) -> Self {
        ScaffoldListEntry {
            template_name,
            template_id,
            author,
            desc,
            tags,
        }
    }
    pub fn matches_query<'a>(&self, queries: &str) -> bool {
        queries.split(" ").all(|query| self.template_name.contains(query) || self.desc.contains(query) || self.tags.iter().any(|tag| tag.text.contains(query)))
    }
}

#[derive(Debug, Default)]
pub struct ScaffoldTabAreas {
    searchbar: Rect,
    list: Rect,
}
impl ScaffoldListEntry {
    fn to_list_item(&self, is_even_item: bool, is_selected: bool) -> widgets::ListItem<'_> {
        let bg_color = if is_selected {
            Color::White
        } else if is_even_item {
            Color::Reset
        } else {
            Color::Indexed(240)
        };
        let contents = Text::from(vec![
            Line::from(vec![
                Span::styled(
                    &self.template_name,
                    Style::new().add_modifier(Modifier::BOLD).bg(bg_color),
                ),
                Span::raw(" ".repeat(200)).bg(bg_color),
            ]),
            Line::from(vec![
                Span::styled(
                    &self.desc,
                    Style::new().add_modifier(Modifier::ITALIC).bg(bg_color),
                ),
                Span::raw(" ".repeat(200)).bg(bg_color),
            ]),
            Line::from(
                self.tags
                    .iter()
                    .flat_map(|a| {
                        a.to_line(bg_color)
                            .spans
                            .into_iter()
                            .chain(iter::once(Span::raw(" ").bg(bg_color)))
                    })
                    .chain(iter::once(Span::raw(" ".repeat(200)).bg(bg_color)))
                    .collect::<Vec<Span>>(),
            ),
        ]);

        widgets::ListItem::new(contents)
    }
}

#[derive(Debug, Default)]
pub enum ScaffoldTabFocus {
    #[default]
    Searchbar,
    List,
}
impl Tab for ScaffoldTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [searchbar_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        let (searchbar_border_color, list_border_color) = if let ScaffoldTabFocus::List = self.focus
        {
            (Color::White, Color::Yellow)
        } else {
            (Color::Yellow, Color::White)
        };
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
            .block(
                Block::bordered()
                    .title("Find Template")
                    .border_style(Style::new().fg(searchbar_border_color)),
            );

        searchbar.render(searchbar_area, buf);

        let list = widgets::List::new(self.list_data.iter().enumerate().map(|(i, list_entry)| {
            list_entry.to_list_item(
                i % 2 == 1,
                self.list_state
                    .selected()
                    .is_some_and(|selected_idx| i == selected_idx),
            )
        }))
        .block(
            Block::bordered()
                .title_bottom(" <ESC> - Exit | <TAB> - Switch Focus | <UP> / <DOWN> - Scroll List ")
                .border_style(Style::new().fg(list_border_color)),
        );
        StatefulWidget::render(list, list_area, buf, &mut self.list_state)
    }
    fn handle_event(&mut self, ev: Event, commands: &mut Commands) {
        match &ev {
            Event::Key(key) => match key.code {
                KeyCode::Tab => {
                    self.focus = match self.focus {
                        ScaffoldTabFocus::Searchbar => ScaffoldTabFocus::List,
                        ScaffoldTabFocus::List => ScaffoldTabFocus::Searchbar,
                    };
                }
                KeyCode::Up => match self.focus {
                    ScaffoldTabFocus::List => {
                        self.list_state.select_previous();
                    }
                    ScaffoldTabFocus::Searchbar => {}
                },
                KeyCode::Down => match self.focus {
                    ScaffoldTabFocus::List => {
                        if self.list_state.selected() != Some(self.list_data.len() - 1) {
                            self.list_state.select_next();
                        }
                    }
                    ScaffoldTabFocus::Searchbar => {}
                },
                KeyCode::Enter => {
                    match self.focus {
                        ScaffoldTabFocus::List => {
                            commands.cache_current_tab();
                            commands.switch_tab_to(ProjectInitTab::new(&self.list_data[self.list_state.selected().unwrap()].template_id));
                        }
                        ScaffoldTabFocus::Searchbar => {
                            self.focus = ScaffoldTabFocus::List;
                        }
                    }
                }
                KeyCode::Esc => {
                    commands.quit();
                }
                _ => {}
            },
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
            ScaffoldTabFocus::List => {}
        }
    }
}
