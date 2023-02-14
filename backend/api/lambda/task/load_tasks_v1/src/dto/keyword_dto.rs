use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]

pub struct KeywordDto {
    pub content: String,
    pub modifiers: Vec<ModifierDto>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ModifierDto {
    NewLine,
    Indentation,
}
