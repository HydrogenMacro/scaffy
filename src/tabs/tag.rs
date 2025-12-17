use ratatui::{prelude::*, widgets};

#[derive(Debug)]
pub enum TagType {
    Language,
    Framework,
    Specialization,
}

#[derive(Debug)]
pub struct Tag {
    pub text: String,
    pub tag_type: TagType,
    pub version: Option<String>
}
impl Tag {
    pub fn new(text: String, tag_type: TagType, version: Option<String>) -> Self {
        Tag { text, tag_type, version }
    }
    pub fn tag_col(&self) -> Color {
        match self.tag_type {
            TagType::Framework => Color::Red,
            TagType::Language => Color::Blue,
            TagType::Specialization => Color::Gray,
        }
    }
    pub fn to_line<'a>(&'a self, bg_color: Color) -> Line<'a> {
        Line::from(vec![
            Span::styled("▐", Style::new().fg(self.tag_col()).bg(bg_color)),
            Span::styled(&self.text, Style::new().bg(self.tag_col())),
            Span::styled("▌", Style::new().fg(self.tag_col()).bg(bg_color)),
            ])
    }
}
