
use ratatui::prelude::*;

use crate::template_info::{ArcStr, TemplateInfoTags};

#[derive(Debug, Clone, Copy)]
pub enum TagType {
    Language,
    Framework,
    Library,
    Misc,
}

#[derive(Debug)]
pub struct Tag {
    pub text: ArcStr,
    pub tag_type: TagType,
    pub version: Option<ArcStr>,
}
impl Tag {
    pub fn new(text: ArcStr, tag_type: TagType, version: Option<ArcStr>) -> Self {
        Tag {
            text,
            tag_type,
            version,
        }
    }
    pub fn tag_col(&self) -> Color {
        match self.tag_type {
            TagType::Framework => Color::Red,
            TagType::Language => Color::Blue,
            TagType::Library => Color::Green,
            TagType::Misc => Color::Gray,
        }
    }
    pub fn to_line<'a>(&'a self, bg_color: Color) -> Line<'a> {
        Line::from(vec![
            Span::styled("▐", Style::new().fg(self.tag_col()).bg(bg_color)),
            Span::styled(&*self.text, Style::new().bg(self.tag_col())),
            Span::styled(
                self.version
                    .as_ref()
                    .map(|version| format!("@{version}"))
                    .unwrap_or(String::new()),
                Style::new().bg(self.tag_col()),
            ),
            Span::styled("▌", Style::new().fg(self.tag_col()).bg(bg_color)),
        ])
    }
}

pub fn parse_template_info_tags(template_info_tags: &TemplateInfoTags) -> Vec<Tag> {
    let map_to_tag = |tag_type: TagType| {
        return move |(name, version): (&ArcStr, &Option<ArcStr>)| {
            Tag::new(name.clone(), tag_type, version.clone())
        };
    };
    [
        template_info_tags
            .languages
            .iter()
            .map(map_to_tag(TagType::Language)),
        template_info_tags
            .frameworks
            .iter()
            .map(map_to_tag(TagType::Framework)),
        template_info_tags
            .libraries
            .iter()
            .map(map_to_tag(TagType::Library)),
        template_info_tags
            .misc
            .iter()
            .map(map_to_tag(TagType::Misc)),
    ]
    .into_iter()
    .flatten()
    .collect()
}
