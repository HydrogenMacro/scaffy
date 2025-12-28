use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use crate::app::Commands;
use crate::input_widget::visual_input_text;
use crate::tabs::Tab;
use crate::template_info::ArcStr;
use crate::template_info::TEMPLATE_INFOS;
use crate::template_info::TemplateStructureDirEntryData;
use crate::template_info::format_template_structure;
use crate::template_info::get_template_file_contents;
use crate::template_info::get_template_structure;
use color_eyre::eyre;
use futures::future::join_all;
use log::info;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::prelude::*;
use ratatui::widgets;
use ratatui::widgets::Block;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Clone, Copy, Debug, Default)]
pub enum PathPageFocus {
    #[default]
    ParentPathInput,
    RootFolderInput
}
#[derive(Clone, Copy, Debug)]
pub enum ProjectInitPage {
    Preview,
    Name,
    Path { focus: PathPageFocus },
    Confirmation,
}
impl ProjectInitPage {
    pub fn page_num(&self) -> usize {
        match self {
            ProjectInitPage::Preview => 1,
            ProjectInitPage::Name => 2,
            ProjectInitPage::Path { .. } => 3,
            ProjectInitPage::Confirmation => 4,
        }
    }
}
impl ProjectInitPage {
    pub fn switch_to_next_page(&mut self) {
        *self = match self {
            ProjectInitPage::Preview => ProjectInitPage::Name,
            ProjectInitPage::Name => ProjectInitPage::Path { focus: PathPageFocus::default() },
            ProjectInitPage::Path {..} => ProjectInitPage::Confirmation,
            ProjectInitPage::Confirmation => ProjectInitPage::Confirmation,
        }
    }

    pub fn switch_to_previous_page(&mut self) {
        *self = match self {
            ProjectInitPage::Preview => ProjectInitPage::Preview,
            ProjectInitPage::Name => ProjectInitPage::Preview,
            ProjectInitPage::Path { ..}=> ProjectInitPage::Name,
            ProjectInitPage::Confirmation => ProjectInitPage::Path { focus: PathPageFocus::default() },
        }
    }
}

pub struct ProjectInitTab {
    current_page: ProjectInitPage,
    template_path: ArcStr,
    project_name_input: Input,
    project_parent_path_input: Input,
    project_root_folder_input: Input,
    should_update_root_folder_val: bool,
    preview_scroll: usize,
}
impl ProjectInitTab {
    pub fn new(template_path: ArcStr) -> Self {
        // TODO: Add prev invocation recall
        ProjectInitTab {
            current_page: ProjectInitPage::Preview,
            template_path: template_path.clone(),
            project_root_folder_input: Input::new(
                env::home_dir().unwrap().to_string_lossy().into(),
            ),
            should_update_root_folder_val: true,
            project_parent_path_input: Input::default(),
            project_name_input: Input::default(),
            preview_scroll: 0,
        }
    }
    pub fn project_path(&self) -> PathBuf {
        let mut path = PathBuf::from(self.project_parent_path_input.value());
        path.push(self.project_root_folder_input.value());
        return path;
    }
}

impl Tab for ProjectInitTab {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let border = Block::bordered()
            .title_bottom(
                " <ESC> - Exit | <ALT + Q> / <SHIFT + TAB> - Prev Page | <ENTER> - Next Page | <UP> / <DOWN> - Move ",
            )
            .title_top(format!(" {} / 4 ", self.current_page.page_num()));
        match self.current_page {
            ProjectInitPage::Preview => {
                let [title_area, preview_area] =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                        .areas(border.inner(area));
                let template_structure = get_template_structure(self.template_path.clone()).unwrap();
                let title = Text::styled(
                    "Template Preview",
                    Style::new().add_modifier(Modifier::BOLD),
                );
                let preview = widgets::Paragraph::new(format_template_structure(&template_structure));
                title.render(title_area, buf);
                preview.render(preview_area, buf);
            }
            ProjectInitPage::Name => {
                let [input_area] =
                    Layout::vertical([Constraint::Length(3)]).areas(border.inner(area));

                let searchbar =
                    widgets::Paragraph::new(visual_input_text(&mut self.project_name_input))
                        .scroll((
                            0,
                            self.project_name_input
                                .visual_scroll(input_area.width as usize)
                                as u16,
                        ))
                        .block(Block::bordered().title("Project Name"));
                searchbar.render(input_area, buf);
            }
            ProjectInitPage::Path { focus } => {
                let [parent_path_input_area, root_folder_input_area, stmt_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(3), Constraint::Length(1)]).areas(border.inner(area));
                
                let (parent_path_input_val, root_folder_input_val) = match focus {
                    PathPageFocus::ParentPathInput => (visual_input_text(&mut self.project_parent_path_input), self.project_root_folder_input.value().into()),
                    PathPageFocus::RootFolderInput => (self.project_parent_path_input.value().into(), visual_input_text(&mut self.project_root_folder_input))
                };
                let parent_path_input_widget =
                    widgets::Paragraph::new(parent_path_input_val)
                        .scroll((
                            0,
                            self.project_parent_path_input
                                .visual_scroll(parent_path_input_area.width as usize)
                                as u16,
                        ))
                        .block(Block::bordered().title("Project Parent Path"));
                    
                let root_folder_input_widget =
                    widgets::Paragraph::new(root_folder_input_val)
                        .scroll((
                            0,
                            self.project_name_input
                                .visual_scroll(root_folder_input_area.width as usize)
                                as u16,
                        ))
                        .block(Block::bordered().title("Project Root Folder Name"));
                let stmt = Text::styled(format!("Your project will be made at {}", self.project_path().to_string_lossy()), Style::new());
                root_folder_input_widget.render(root_folder_input_area, buf);
                parent_path_input_widget.render(parent_path_input_area, buf);
                stmt.render(stmt_area, buf);
            }
            ProjectInitPage::Confirmation => {
                let [paragraph_area] =
                    Layout::vertical([Constraint::Fill(1)]).areas(border.inner(area));
                let paragraph = widgets::Paragraph::new("Confirm creation?\nPress <ENTER> to confirm.\n Press <ESC> to exit.");
                paragraph.render(paragraph_area, buf);
            }
        }

        border.render(area, buf);
    }

    fn handle_event(&mut self, ev: Event, commands: &mut Commands) {
        match &ev {
            Event::Key(key_ev) => match key_ev.code {
                KeyCode::Char('q') if key_ev.modifiers.contains(KeyModifiers::ALT) => {
                    self.current_page.switch_to_previous_page();
                }
                KeyCode::Tab if key_ev.modifiers.contains(KeyModifiers::SHIFT) => {
                    self.current_page.switch_to_previous_page();
                }

                KeyCode::Enter => match &self.current_page {
                    ProjectInitPage::Preview | ProjectInitPage::Name | ProjectInitPage::Path {..}=> {
                        self.current_page.switch_to_next_page();
                    }
                    ProjectInitPage::Confirmation => {
                        init_project(
                            self.template_path.clone(),
                            self.project_name_input.value(),
                            &self.project_path(),
                        )
                        .unwrap();
                        commands.quit();
                    }
                },
                KeyCode::Esc => {
                    commands.switch_tab_to_cached();
                }
                _ => {}
            },
            _ => {}
        }
        match self.current_page {
            ProjectInitPage::Name => {
                self.project_name_input.handle_event(&ev);
            }
            ProjectInitPage::Confirmation => {}
            ProjectInitPage::Preview => match &ev {
                Event::Key(key_ev) => match key_ev.code {
                    KeyCode::Down => {}
                    KeyCode::Up => {}
                    _ => {}
                },
                _ => {}
            },
            ProjectInitPage::Path { focus } => {
                match focus {
                    PathPageFocus::ParentPathInput => self.project_parent_path_input.handle_event(&ev),
                    PathPageFocus::RootFolderInput => self.project_root_folder_input.handle_event(&ev),
                };
            }
        }
    }
}

fn init_project(
    template_path: ArcStr,
    project_name: &str,
    project_root_dir: &Path,
) -> eyre::Result<()> {
    let template_structure = get_template_structure(template_path.clone())?;
    let mut stack: Vec<(ArcStr, TemplateStructureDirEntryData, Vec<ArcStr>)> = template_structure
        .into_iter()
        .map(|(dir_entry_name, dir_entry)| (dir_entry_name, dir_entry, vec![]))
        .collect();

    let mut tasks = vec![];
    while let Some((dir_entry_name, dir_entry, parent_path)) = stack.pop() {
        let joined_parent_path: ArcStr = Arc::from(parent_path.join("/"));
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
                let dir_entry_name = dir_entry_name.clone();
                let template_path = template_path.clone();

                tasks.push(Box::pin(async move {
                    let z = get_template_file_contents(
                        template_path,
                        joined_parent_path,
                        dir_entry_name.clone(),
                    )
                    .await?;
                    info!("resolved {:?}", (&dir_entry_name, &parent_path, z.len()));
                    Ok::<(), eyre::Error>(())
                }));
            }
        }
    }
    let results: Vec<()> = smol::block_on(join_all(&mut tasks))
        .into_iter()
        .collect::<eyre::Result<Vec<()>>>()?;
    info!("{:?}", project_root_dir);
    Ok(())
}
