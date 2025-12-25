use crate::app::Commands;
use crate::tabs::Tab;
use crate::template_info::TEMPLATE_INFOS;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::prelude::*;
use ratatui::widgets;
use ratatui::widgets::Block;

#[derive(Clone, Copy, Debug)]
pub enum ProjectInitPage {
    Preview = 0,
    Name,
    Path,
    Confirmation
}
impl ProjectInitPage {
    pub fn switch_to_next_page(&mut self) {
        *self = match self {
            ProjectInitPage::Preview => ProjectInitPage::Name,
            ProjectInitPage::Name => ProjectInitPage::Path,
            ProjectInitPage::Path => ProjectInitPage::Confirmation,
            ProjectInitPage::Confirmation => ProjectInitPage::Confirmation,
        }
    }

    pub fn switch_to_previous_page(&mut self) {
        *self = match self {
            ProjectInitPage::Preview => ProjectInitPage::Preview,
            ProjectInitPage::Name => ProjectInitPage::Preview,
            ProjectInitPage::Path => ProjectInitPage::Name,
            ProjectInitPage::Confirmation => ProjectInitPage::Path,
        }
    }
}

pub struct ProjectInitTab {
    current_page: ProjectInitPage
}
impl ProjectInitTab {
    pub fn new(template_dir: &str) -> Self {
        TEMPLATE_INFOS.with(|template_infos| {
            let template_infos = template_infos.borrow();
            let template_info  = template_infos.get(template_dir).unwrap();
            template_info.path.clone();
            ProjectInitTab {
                current_page: ProjectInitPage::Preview,
            }
        })
    }
}

impl Tab for ProjectInitTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let content =
            widgets::Paragraph::new(vec![])
                .block(Block::bordered().title_bottom(
                    " <ESC> - Exit | <SHIFT + TAB> - Previous Page | <ENTER> - Next Page / Submit ",
                ).title_top(format!(" {} / 4 ", self.current_page as usize)));
        content.render(area, buf);
    }
    fn handle_event(&mut self, ev: Event, commands: &mut Commands) {
        match ev {
            Event::Key(key_ev) => match key_ev.code {
                KeyCode::Tab => {
                    if key_ev.modifiers.contains(KeyModifiers::SHIFT) {
                        self.current_page.switch_to_previous_page();
                    }
                }
                KeyCode::Enter => {
                    match &self.current_page {
                        ProjectInitPage::Preview | ProjectInitPage::Name | ProjectInitPage::Path => {
                            self.current_page.switch_to_next_page();
                        }
                        ProjectInitPage::Confirmation => {
                            
                        },
                    }
                }
                KeyCode::Esc => {
                    commands.switch_tab_to_cached();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
