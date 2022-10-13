#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DocumentDiagnostics {
    pub path: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub message: String,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Severity {
    ERROR,
    WARNING,
    INFO,
    HINT,
}
