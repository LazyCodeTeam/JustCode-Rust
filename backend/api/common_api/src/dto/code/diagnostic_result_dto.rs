use code_domain::model::diagnostic_result::{
    Diagnostic, DocumentDiagnostics, Position, Range, Severity,
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub struct DocumentDiagnosticsDto {
    pub path: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub struct DiagnosticDto {
    pub code: String,
    pub severity: SeverityDto,
    pub message: String,
    pub range: RangeDto,
}

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub struct PositionDto {
    pub line: u64,
    pub column: u64,
}

#[derive(Serialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum SeverityDto {
    ERROR,
    WARNING,
    INFO,
    HINT,
    UNKNOWN,
}

impl From<DocumentDiagnostics> for DocumentDiagnosticsDto {
    fn from(document_diagnostic: DocumentDiagnostics) -> Self {
        Self {
            path: document_diagnostic.path,
            diagnostics: document_diagnostic
                .diagnostics
                .into_iter()
                .map(DiagnosticDto::from)
                .collect(),
        }
    }
}

impl From<Diagnostic> for DiagnosticDto {
    fn from(diagnostic: Diagnostic) -> Self {
        Self {
            code: diagnostic.code,
            severity: diagnostic.severity.into(),
            message: diagnostic.message,
            range: diagnostic.range.into(),
        }
    }
}

impl From<Range> for RangeDto {
    fn from(range: Range) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl From<Position> for PositionDto {
    fn from(position: Position) -> Self {
        Self {
            line: position.line,
            column: position.column,
        }
    }
}

impl From<Severity> for SeverityDto {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::ERROR => SeverityDto::ERROR,
            Severity::WARNING => SeverityDto::WARNING,
            Severity::INFO => SeverityDto::INFO,
            Severity::HINT => SeverityDto::HINT,
            Severity::UNKNOWN => SeverityDto::UNKNOWN,
        }
    }
}

#[cfg(test)]
mod test {
    use code_domain::model::diagnostic_result::Position;

    use super::*;

    #[test]
    fn from_document_diagnostic() {
        let document_diagnostic = DocumentDiagnostics {
            path: "path".to_owned(),
            diagnostics: vec![
                Diagnostic {
                    code: "code_1".to_owned(),
                    severity: Severity::ERROR,
                    message: "message_1".to_owned(),
                    range: Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 1 },
                    },
                },
                Diagnostic {
                    code: "code_2".to_owned(),
                    severity: Severity::INFO,
                    message: "message_2".to_owned(),
                    range: Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 1 },
                    },
                },
            ],
        };

        let dto = DocumentDiagnosticsDto::from(document_diagnostic.clone());

        assert_eq!(
            dto,
            DocumentDiagnosticsDto {
                path: "path".to_owned(),
                diagnostics: document_diagnostic
                    .diagnostics
                    .into_iter()
                    .map(DiagnosticDto::from)
                    .collect()
            }
        )
    }

    #[test]
    fn from_diagnostic() {
        let diagnostic = Diagnostic {
            code: "code".to_owned(),
            severity: Severity::ERROR,
            message: "message".to_owned(),
            range: Range {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 1 },
            },
        };

        let dto = DiagnosticDto::from(diagnostic.clone());

        assert_eq!(
            dto,
            DiagnosticDto {
                code: "code".to_owned(),
                severity: diagnostic.severity.into(),
                message: "message".to_owned(),
                range: diagnostic.range.into(),
            }
        )
    }

    #[test]
    fn from_range() {
        let range = Range {
            start: Position { line: 1, column: 2 },
            end: Position { line: 3, column: 4 },
        };
        let dto = RangeDto::from(range.clone());

        assert_eq!(
            dto,
            RangeDto {
                start: range.start.into(),
                end: range.end.into(),
            }
        )
    }

    #[test]
    fn from_severity() {
        let severities = vec![
            Severity::ERROR,
            Severity::WARNING,
            Severity::INFO,
            Severity::HINT,
            Severity::UNKNOWN,
        ];
        let expected_dtos = vec![
            SeverityDto::ERROR,
            SeverityDto::WARNING,
            SeverityDto::INFO,
            SeverityDto::HINT,
            SeverityDto::UNKNOWN,
        ];

        let result: Vec<SeverityDto> = severities.into_iter().map(SeverityDto::from).collect();

        assert_eq!(result, expected_dtos)
    }

    #[test]
    fn from_position() {
        let position = Position { line: 1, column: 2 };
        let result = PositionDto::from(position);

        assert_eq!(result, PositionDto { line: 1, column: 2 })
    }
}
