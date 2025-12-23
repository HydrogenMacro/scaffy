use reqwest::blocking;
use serde::Deserialize;
use serde_json::Map;
use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::{LazyLock, Mutex},
    thread::LocalKey,
};

const SOURCES: &'static [&'static str] =
    &["https://raw.githubusercontent.com/HydrogenMacro/scaffy/refs/heads/master/templates"];
thread_local! {
pub static TEMPLATE_INFO_CACHE: RefCell<LazyCell<HashMap<Rc<str>, TemplateInfo>>> = RefCell::new(LazyCell::new(|| HashMap::new()));
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfo {
    name: String,
    path: Rc<str>,
    author: String,
    description: String,
    tags: TemplateInfoTags,
    projectDetailsInjectionFiles: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct TemplateInfoTags {
    pub languages: HashMap<Rc<str>, String>,
    pub frameworks: HashMap<Rc<str>, String>,
    pub libraries: HashMap<Rc<str>, String>,
    pub misc: HashMap<Rc<str>, String>,
}
pub fn fetch_template_info() {
    for &source in SOURCES {
        let unparsed = blocking::get(format!("{}/templates.json", source))
            .unwrap()
            .text()
            .unwrap();
        let mut template_infos = serde_json::from_str::<Vec<TemplateInfo>>(&unparsed).unwrap();
        TEMPLATE_INFO_CACHE.with(|template_info_cache| {
            template_info_cache
                .borrow_mut()
                .insert(template_infos[0].path.clone(), template_infos.pop().unwrap());
        });
    }
}

#[test]
pub fn a() {
    fetch_template_info();
    TEMPLATE_INFO_CACHE.with(|a| {
        dbg!(a.borrow());
    });
}
