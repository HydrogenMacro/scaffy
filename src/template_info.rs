use color_eyre::eyre;
use itertools::Itertools;
use log::info;
use reqwest::blocking;
use serde::Deserialize;
use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    rc::Rc, sync::Arc,
};

const SOURCE: &'static str = "https://cdn.jsdelivr.net/gh/hydrogenmacro/scaffy@master/templates";
thread_local! {
pub static TEMPLATE_INFOS: RefCell<LazyCell<HashMap<Rc<str>, TemplateInfo>>> = RefCell::new(LazyCell::new(|| HashMap::new()));
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemplateInfo {
    pub name: Rc<str>,
    pub path: Rc<str>,
    pub author: Rc<str>,
    pub description: Rc<str>,
    pub tags: TemplateInfoTags,
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfoTags {
    pub languages: HashMap<Rc<str>, Option<Rc<str>>>,
    pub frameworks: HashMap<Rc<str>, Option<Rc<str>>>,
    pub libraries: HashMap<Rc<str>, Option<Rc<str>>>,
    pub misc: HashMap<Rc<str>, Option<Rc<str>>>,
}
pub fn fetch_template_info() -> eyre::Result<()> {
    let unparsed = blocking::get(format!("{}/templates.json", SOURCE))?.text()?;
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "type")]
pub enum TemplateStructureDirEntryData {
    Folder {
        inject_project_info: bool,
        children: HashMap<Rc<str>, TemplateStructureDirEntryData>,
    },
    File {
        inject_project_info: bool,
    },
}

pub type TemplateStructure = HashMap<Rc<str>, TemplateStructureDirEntryData>;
pub fn get_template_structure(template_path: impl AsRef<str>) -> eyre::Result<TemplateStructure> {
    let unparsed_data = blocking::get(format!(
        "{}/__scaffy_template_contents/{}.json",
        SOURCE,
        template_path.as_ref()
    ))?
    .text()?;

    let template_info = serde_json::from_str::<TemplateStructure>(&unparsed_data)?;
    Ok(template_info)
}

pub async fn get_template_file_contents(template_path: impl AsRef<str>, file_parent_path: Rc<str>, file_name: Rc<str>) -> eyre::Result<String> {
    let file_text = reqwest::get(format!(
        "{}/{}{}/{}",
        SOURCE,
        template_path.as_ref(),
        file_parent_path,
        file_name
    )).await?
    .text().await?;

    return Ok(file_text);
}