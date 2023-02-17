#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Keyword {
    pub id: u16,
    pub content: String,
    pub modifiers: Vec<KeywordModifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordModifier {
    NewLine,
    AddIndentation,
    RemoveIndentation,
}
