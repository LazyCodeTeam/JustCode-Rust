use serde::Deserialize;
use task_domain::model::hint::Hint;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
pub struct HintDto {
    pub content: String,
}

impl From<HintDto> for Hint {
    fn from(dto: HintDto) -> Self {
        Hint {
            content: dto.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let dto = HintDto {
            content: "content".to_string(),
        };
        let expected = Hint {
            content: "content".to_string(),
        };
        assert_eq!(expected, Hint::from(dto));
    }
}
