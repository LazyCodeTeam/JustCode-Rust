use code_domain::model::code_file::CodeFile;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct FileDto {
    pub path: String,
    pub content: String,
}

impl From<CodeFile> for FileDto {
    fn from(model: CodeFile) -> Self {
        Self {
            path: model.path,
            content: model.content,
        }
    }
}

impl From<FileDto> for CodeFile {
    fn from(dto: FileDto) -> Self {
        CodeFile {
            path: dto.path,
            content: dto.content,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_model() {
        let path = "test_path".to_owned();
        let content = "test_content".to_owned();
        let model = CodeFile {
            path: path.clone(),
            content: content.clone(),
        };

        assert_eq!(FileDto::from(model), FileDto { path, content })
    }

    #[test]
    fn into_model() {
        let path = "test_path".to_owned();
        let content = "test_content".to_owned();
        let dto = FileDto {
            path: path.clone(),
            content: content.clone(),
        };

        assert_eq!(CodeFile::from(dto), CodeFile { path, content })
    }
}
