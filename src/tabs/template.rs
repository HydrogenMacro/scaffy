use std::rc::Rc;

pub struct ProjectTemplateData {
    pub files: Vec<ProjectTemplateDirEntryData>
}

pub enum ProjectTemplateDirEntryData {
    Folder {
        name: String,
        /// `None` if it would be the same as `name`
        normalized_name: Option<String>,
        children: Vec<ProjectTemplateDirEntryData>
    },
    File {
        name: String
    }
}
