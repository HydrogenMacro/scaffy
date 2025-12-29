use color_eyre::eyre;
use itertools::Itertools;
use log::info;
use serde::Deserialize;
use std::{
    cell::{LazyCell, RefCell},
    cmp::Ordering,
    collections::HashMap,
    sync::Arc,
};

use crate::string_ops::scaffy_string_replacement;

const SOURCE: &'static str = "https://cdn.jsdelivr.net/gh/hydrogenmacro/scaffy@master/templates";

thread_local! {
pub static TEMPLATE_INFOS: RefCell<LazyCell<HashMap<ArcStr, TemplateInfo>>> = RefCell::new(LazyCell::new(|| HashMap::new()));
}

pub type ArcStr = Arc<str>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemplateInfo {
    pub name: ArcStr,
    pub path: ArcStr,
    pub author: ArcStr,
    pub description: ArcStr,
    pub tags: TemplateInfoTags,
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfoTags {
    pub languages: HashMap<ArcStr, Option<ArcStr>>,
    pub frameworks: HashMap<ArcStr, Option<ArcStr>>,
    pub libraries: HashMap<ArcStr, Option<ArcStr>>,
    pub misc: HashMap<ArcStr, Option<ArcStr>>,
}
pub fn fetch_template_info() -> eyre::Result<()> {
    let unparsed = smol::block_on(surf::get(format!("{}/templates.json", SOURCE)).recv_string())
        .map_err(eyre::Error::msg)?;
    let template_infos = serde_json::from_str::<Vec<TemplateInfo>>(&unparsed)?;

    TEMPLATE_INFOS.with(|template_info_cache| {
        for template_info in template_infos {
            template_info_cache
                .borrow_mut()
                .insert(template_info.path.clone(), template_info);
        }
    });

    Ok(())
}

#[derive(Deserialize, Clone)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "type"
)]
pub enum TemplateStructureDirEntryData {
    Folder {
        inject_project_info: bool,
        children: HashMap<ArcStr, TemplateStructureDirEntryData>,
    },
    File {
        inject_project_info: bool,
    },
}

pub type TemplateStructure = HashMap<ArcStr, TemplateStructureDirEntryData>;
pub fn format_template_structure(template_structure: &TemplateStructure, project_name: impl AsRef<str>) -> String {
    let template_entry_sorting_fn = |&(dir_entry_name_1, dir_entry_1, _): &(
        &Arc<str>,
        &TemplateStructureDirEntryData,
        usize,
    ),
                                     &(dir_entry_name_2, dir_entry_2, _): &(
        &Arc<str>,
        &TemplateStructureDirEntryData,
        usize,
    )| {
        let mut val1 = 0;
        let mut val2 = 0;
        if let TemplateStructureDirEntryData::Folder { .. } = dir_entry_1 {
            val1 = 1;
        }
        if let TemplateStructureDirEntryData::Folder { .. } = dir_entry_2 {
            val2 = 1;
        }
        match val1.cmp(&val2) {
            Ordering::Equal => dir_entry_name_1.cmp(&dir_entry_name_2).reverse(),
            other => other,
        }
    };
    let mut output = String::new();
    let mut stack = template_structure
        .iter()
        .map(|(k, v)| (k, v, 0usize))
        .collect::<Vec<_>>();
    stack.sort_unstable_by(template_entry_sorting_fn);

    while let Some((dir_entry_name, dir_entry, nest_level)) = stack.pop() {
        let formatted_dir_entry_name = scaffy_string_replacement(dir_entry_name, project_name.as_ref());
        match dir_entry {
            TemplateStructureDirEntryData::Folder {
                inject_project_info,
                children,
            } => {
                let mut children_entries = children
                    .iter()
                    .map(|(k, v)| (k, v, nest_level + 1))
                    .collect::<Vec<_>>();
                children_entries.sort_unstable_by(template_entry_sorting_fn);
                stack.extend(children_entries);
                if nest_level == 0 {
                    output.push_str("🖿 ");
                    output.push_str(&formatted_dir_entry_name);
                } else {
                    let line = format!("{}🖿 {}", " ".repeat(nest_level * 4), &formatted_dir_entry_name);
                    output.push_str(&line);
                }
            }
            TemplateStructureDirEntryData::File {
                inject_project_info,
            } => {
                if nest_level == 0 {
                    output.push_str("🗎 ");
                    output.push_str(dir_entry_name);
                } else {
                    let line = format!("{}🗎 {}", " ".repeat(nest_level * 4), &*dir_entry_name);
                    output.push_str(&line);
                }
            }
        }
        output.push('\n');
    }
    return output;
}

thread_local! {
pub static TEMPLATE_STRUCTURE_CACHE: RefCell<LazyCell<HashMap<ArcStr, TemplateStructure>>> = RefCell::new(LazyCell::new(|| HashMap::new()));
}

pub fn get_template_structure(template_path: ArcStr) -> eyre::Result<TemplateStructure> {
    TEMPLATE_STRUCTURE_CACHE.with(|template_structure_cache| {
        let mut template_structure_cache = template_structure_cache.borrow_mut();

        if !template_structure_cache.contains_key(&template_path) {
            let unparsed_data = smol::block_on(
                surf::get(format!(
                    "{}/__scaffy_template_contents/{}.json",
                    SOURCE,
                    template_path.as_ref()
                ))
                .recv_string(),
            )
            .map_err(eyre::Error::msg)?;

            let template_info = serde_json::from_str::<TemplateStructure>(&unparsed_data)?;
            template_structure_cache.insert(template_path.clone(), template_info);
        }
        let t = template_structure_cache
            .get(&template_path)
            .unwrap()
            .clone();
        return Ok(t);
    })
}

pub async fn get_template_file_contents(
    template_path: impl AsRef<str>,
    file_parent_path: ArcStr,
    file_name: ArcStr,
) -> eyre::Result<String> {
    let file_text = surf::get(format!(
        "{}/{}{}/{}",
        SOURCE,
        template_path.as_ref(),
        file_parent_path,
        file_name
    ))
    .recv_string()
    .await
    .map_err(eyre::Error::msg)?;

    return Ok(file_text);
}
