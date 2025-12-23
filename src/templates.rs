use color_eyre::eyre;
use reqwest::blocking;
use serde::Deserialize;
use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

const SOURCES: &'static [&'static str] =
    &["https://raw.githubusercontent.com/HydrogenMacro/scaffy/refs/heads/master/templates"];
thread_local! {
pub static TEMPLATE_INFOS: RefCell<LazyCell<HashMap<Rc<str>, TemplateInfo>>> = RefCell::new(LazyCell::new(|| HashMap::new()));
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfo {
    pub name: Rc<str>,
    pub path: Rc<str>,
    pub author: Rc<str>,
    pub description: Rc<str>,
    pub tags: TemplateInfoTags,
    pub project_details_injection_files: Vec<Rc<str>>,
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfoTags {
    pub languages: HashMap<Rc<str>, Option<Rc<str>>>,
    pub frameworks: HashMap<Rc<str>, Option<Rc<str>>>,
    pub libraries: HashMap<Rc<str>, Option<Rc<str>>>,
    pub misc: HashMap<Rc<str>, Option<Rc<str>>>,
}
pub fn fetch_template_info() -> eyre::Result<()> {
    for &source in SOURCES {
        let unparsed = blocking::get(format!("{}/templates.json", source))?.text()?;
        let template_infos = serde_json::from_str::<Vec<TemplateInfo>>(&unparsed)?;

        TEMPLATE_INFOS.with(|template_info_cache| {
            for template_info in template_infos {
                template_info_cache
                    .borrow_mut()
                    .insert(template_info.path.clone(), template_info);
            }
        });
    }
    Ok(())
}
