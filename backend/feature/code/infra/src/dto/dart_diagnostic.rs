use code_domain::model::diagnostic_result::{
    Diagnostic, DocumentDiagnostics, Position, Range, Severity,
};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticResultDto {
    pub version: u8,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticDto {
    pub code: String,
    pub severity: SeverityDto,
    pub problem_message: String,
    pub correction_message: Option<String>,
    pub documentation: Option<String>,
    pub location: LocationDto,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SeverityDto(String);

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocationDto {
    pub file: String,
    pub range: RangeDto,
}
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    pub offset: u64,
    pub line: u64,
    pub column: u64,
}

impl From<DiagnosticResultDto> for Vec<DocumentDiagnostics> {
    fn from(dto: DiagnosticResultDto) -> Self {
        dto.diagnostics
            .into_iter()
            .into_group_map_by(|diagnostic| diagnostic.location.file.clone())
            .into_iter()
            .map(|(key, value)| DocumentDiagnostics {
                path: key,
                diagnostics: value.into_iter().map(Diagnostic::from).collect(),
            })
            .collect()
    }
}

impl From<DiagnosticDto> for Diagnostic {
    fn from(dto: DiagnosticDto) -> Self {
        Self {
            code: dto.code,
            severity: dto.severity.into(),
            message: format!(
                "{}{}{}",
                dto.problem_message,
                dto.correction_message
                    .map(|s| format!(" - {}", s))
                    .unwrap_or_else(|| "".to_owned()),
                dto.documentation
                    .map(|s| format!(" - {}", s))
                    .unwrap_or_else(|| "".to_owned()),
            ),
            range: dto.location.range.into(),
        }
    }
}

impl From<PositionDto> for Position {
    fn from(dto: PositionDto) -> Self {
        Self {
            line: dto.line,
            column: dto.column,
        }
    }
}

impl From<RangeDto> for Range {
    fn from(dto: RangeDto) -> Self {
        Self {
            start: dto.start.into(),
            end: dto.end.into(),
        }
    }
}

impl From<SeverityDto> for Severity {
    fn from(dto: SeverityDto) -> Self {
        let severity = dto.0;
        match severity.as_str() {
            "HINT" => Self::HINT,
            "INFO" => Self::INFO,
            "WARNING" => Self::WARNING,
            "ERROR" => Self::ERROR,
            severity => {
                log::warn!("Unknown severity: {severity:?}");
                Self::UNKNOWN
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_diagnostic_result_dto() {
        let diagnostic_dto_1 = DiagnosticDto {
            code: "diagnostic_code_1".to_owned(),
            severity: SeverityDto("INFO".to_owned()),
            problem_message: "problem_1".to_owned(),
            correction_message: Some("correction_1".to_owned()),
            documentation: Some("documentation_1".to_owned()),
            location: LocationDto {
                file: "file".to_owned(),
                range: RangeDto {
                    start: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                    end: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                },
            },
        };
        let diagnostic_dto_2 = DiagnosticDto {
            code: "diagnostic_code_2".to_owned(),
            severity: SeverityDto("INFO".to_owned()),
            problem_message: "problem_2".to_owned(),
            correction_message: Some("correction_2".to_owned()),
            documentation: Some("documentation_2".to_owned()),
            location: LocationDto {
                file: "file_2".to_owned(),
                range: RangeDto {
                    start: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                    end: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                },
            },
        };
        let diagnostic_dto_3 = DiagnosticDto {
            code: "diagnostic_code_3".to_owned(),
            severity: SeverityDto("INFO".to_owned()),
            problem_message: "problem_3".to_owned(),
            correction_message: Some("correction_3".to_owned()),
            documentation: Some("documentation_3".to_owned()),
            location: LocationDto {
                file: "file".to_owned(),
                range: RangeDto {
                    start: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                    end: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                },
            },
        };
        let dto = DiagnosticResultDto {
            version: 1,
            diagnostics: vec![
                diagnostic_dto_1.clone(),
                diagnostic_dto_2.clone(),
                diagnostic_dto_3.clone(),
            ],
        };

        let mut result: Vec<DocumentDiagnostics> = dto.into();
        let mut expected = vec![
            DocumentDiagnostics {
                path: "file".to_owned(),
                diagnostics: vec![diagnostic_dto_1.into(), diagnostic_dto_3.into()],
            },
            DocumentDiagnostics {
                path: "file_2".to_owned(),
                diagnostics: vec![diagnostic_dto_2.into()],
            },
        ];
        result.sort_by_key(|item| item.path.clone());
        expected.sort_by_key(|item| item.path.clone());

        assert_eq!(result, expected)
    }

    #[test]
    fn from_diagnostic_dto() {
        let dto = DiagnosticDto {
            code: "diagnostic_code".to_owned(),
            severity: SeverityDto("INFO".to_owned()),
            problem_message: "problem".to_owned(),
            correction_message: Some("correction".to_owned()),
            documentation: Some("documentation".to_owned()),
            location: LocationDto {
                file: "file".to_owned(),
                range: RangeDto {
                    start: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                    end: PositionDto {
                        offset: 1,
                        line: 1,
                        column: 1,
                    },
                },
            },
        };

        let result = Diagnostic::from(dto.clone());
        assert_eq!(
            result,
            Diagnostic {
                code: dto.code,
                severity: dto.severity.into(),
                message: "problem - correction - documentation".to_owned(),
                range: dto.location.range.into()
            }
        )
    }

    #[test]
    fn from_severity_dto() {
        let severities = ["HINT", "INFO", "WARNING", "ERROR", "aaaaa"];
        let expected = vec![
            Severity::HINT,
            Severity::INFO,
            Severity::WARNING,
            Severity::ERROR,
            Severity::UNKNOWN,
        ];
        let result: Vec<Severity> = severities
            .into_iter()
            .map(|s| SeverityDto(s.to_owned()))
            .map(Severity::from)
            .collect();

        assert_eq!(result, expected)
    }

    #[test]
    fn from_postion_dto() {
        let dto = PositionDto {
            line: 1,
            column: 2,
            offset: 3,
        };

        let expected = Position { line: 1, column: 2 };

        assert_eq!(Position::from(dto), expected)
    }

    #[test]
    fn from_range_dto() {
        let start_dto = PositionDto {
            line: 1,
            column: 2,
            offset: 3,
        };
        let end_dto = PositionDto {
            line: 4,
            column: 5,
            offset: 6,
        };
        let range_dto = RangeDto {
            start: start_dto.clone(),
            end: end_dto.clone(),
        };

        assert_eq!(
            Range::from(range_dto),
            Range {
                start: start_dto.into(),
                end: end_dto.into()
            }
        )
    }
}
