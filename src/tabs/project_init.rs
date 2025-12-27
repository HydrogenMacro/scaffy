use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::app::Commands;
use crate::tabs::Tab;
use crate::template_info::TEMPLATE_INFOS;
use crate::template_info::TemplateStructureDirEntryData;
use crate::template_info::get_template_file_contents;
use crate::template_info::get_template_structure;
use ::futures::future::join_all;
use ::futures::task::SpawnExt;
use color_eyre::eyre;
use futures_scopes::local::LocalScope;
use log::info;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::prelude::*;
use ratatui::widgets;
use ratatui::widgets::Block;
use smol::Executor;
use smol::block_on;
use smol::future;
use smol::lock::futures;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Clone, Copy, Debug)]
pub enum ProjectInitPage {
    Preview = 0,
    Name,
    Path,
    Confirmation,
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
    current_page: ProjectInitPage,
    template_path: Rc<str>,
    project_name: String,
    project_root_dir: PathBuf,
    input: Input,
}
impl ProjectInitTab {
    pub fn new(template_path: Rc<str>) -> Self {
        TEMPLATE_INFOS.with(|template_infos| {
            let template_infos = template_infos.borrow();
            let template_info = template_infos.get(&*template_path).unwrap();
            // TODO: Add prev invocation recall
            let project_root_dir = env::current_dir().unwrap();
            ProjectInitTab {
                current_page: ProjectInitPage::Preview,
                template_path: template_path.clone(),
                project_name: String::new(),
                project_root_dir,
                input: Input::default(),
            }
        })
    }
    fn save_page(&mut self) {
        match self.current_page {
            ProjectInitPage::Name => {
                self.project_name = self.input.value().to_owned();
            }
            ProjectInitPage::Path => {
                self.project_root_dir = self.input.value().into();
            }
            _ => {}
        }
    }
    fn restore_page(&mut self) {
        match self.current_page {
            ProjectInitPage::Name => {
                self.input = Input::new(self.project_name.clone());
            }
            ProjectInitPage::Path => {
                self.input = Input::new(self.project_root_dir.to_string_lossy().into());
            }
            _ => {}
        }
    }
}

impl Tab for ProjectInitTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let border = Block::bordered()
            .title_bottom(
                " <ESC> - Exit | <SHIFT + TAB> - Previous Page | <ENTER> - Next Page / Submit ",
            )
            .title_top(format!(" {} / 4 ", self.current_page as usize + 1));
        let [title_area, input_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
                .areas(border.inner(area));
        let mut searchbar_text = self.input.value().to_string();
        if searchbar_text.len() == self.input.cursor() {
            searchbar_text.push('\u{2588}');
        } else {
            let cursor_pos = self.input.cursor();
            searchbar_text.replace_range(cursor_pos..=cursor_pos, "\u{2588}");
        }
        let mut searchbar = widgets::Paragraph::new(searchbar_text)
            .scroll((
                0,
                self.input.visual_scroll(input_area.width as usize) as u16,
            ))
            .block(Block::bordered().title("Title"));
        searchbar.render(input_area, buf);
        border.render(area, buf);
    }

    fn handle_event(&mut self, ev: Event, commands: &mut Commands) {
        match &ev {
            Event::Key(key_ev) => match key_ev.code {
                KeyCode::Tab => {
                    if key_ev.modifiers.contains(KeyModifiers::SHIFT) {
                        self.save_page();
                        self.current_page.switch_to_previous_page();
                        self.restore_page();
                    }
                }
                KeyCode::Enter => match &self.current_page {
                    ProjectInitPage::Preview | ProjectInitPage::Name | ProjectInitPage::Path => {
                        self.save_page();
                        self.current_page.switch_to_next_page();
                        self.restore_page();
                    }
                    ProjectInitPage::Confirmation => {
                        init_project(
                            self.template_path.clone(),
                            &self.project_name,
                            &self.project_root_dir,
                        )
                        .unwrap();
                    }
                },
                KeyCode::Esc => {
                    commands.switch_tab_to_cached();
                }
                _ => {}
            },
            _ => {}
        }
        self.input.handle_event(&ev);
    }
}

fn init_project(
    template_path: Rc<str>,
    project_name: &str,
    project_root_dir: &Path,
) -> eyre::Result<()> {
    let template_structure = get_template_structure(&template_path)?;
    let mut stack: Vec<(Rc<str>, TemplateStructureDirEntryData, Vec<Rc<str>>)> = template_structure
        .into_iter()
        .map(|(dir_entry_name, dir_entry)| (dir_entry_name, dir_entry, vec![]))
        .collect();

    let mut ex = Executor::new();
    let mut local_scope = LocalScope::<()>::new();
    while let Some((dir_entry_name, dir_entry, parent_path)) = stack.pop() {
        let joined_parent_path: Rc<str> = Rc::from(parent_path.join("/"));
        match dir_entry {
            TemplateStructureDirEntryData::Folder {
                inject_project_info,
                children,
            } => {
                stack.extend(
                    children
                        .into_iter()
                        .map(|(sub_dir_entry_name, sub_dir_entry)| {
                            let mut sub_dir_parent_path = parent_path.clone();
                            sub_dir_parent_path.push(dir_entry_name.clone());
                            (sub_dir_entry_name, sub_dir_entry, sub_dir_parent_path)
                        }),
                );
            }
            TemplateStructureDirEntryData::File {
                inject_project_info,
            } => {
                let joined_parent_path = joined_parent_path.clone();
                let spawner = local_scope.spawner();
                let dir_entry_name = dir_entry_name.clone();
                let template_path = template_path.clone();
                spawner
                    .spawn_local_scoped(async move {
                        let z = get_template_file_contents(
                            template_path,
                            joined_parent_path,
                            dir_entry_name.clone(),
                        )
                        .await
                        .unwrap();
                        info!("{:?}", (&dir_entry_name, &parent_path, z.len()));
                    })
                    .unwrap();
            }
        }
    }
    block_on(local_scope.until_empty());
    Ok(())
}
