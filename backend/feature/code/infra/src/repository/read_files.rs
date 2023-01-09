use std::{io::Read, path::Path};

use code_domain::model::code_file::CodeFile;
use common_domain::error::{Error, Result};

pub async fn read_files(path: &Path) -> Result<Vec<CodeFile>> {
    read_files_sync(path, path)
}

fn read_files_sync(base_path: &Path, root_path: &Path) -> Result<Vec<CodeFile>> {
    let subdirs = std::fs::read_dir(base_path)
        .map_err(|e| Error::unknown(format!("Failed to read subdirs of {base_path:?} ({e:?})")))?;

    let mut files = Vec::new();

    for entry in subdirs {
        let entry =
            entry.map_err(|e| Error::unknown(format!("Failed to read data of file ({e:?})")))?;
        let file_type = entry.file_type().map_err(|e| {
            Error::unknown(format!("Failed to read file type of {entry:?} ({e:?})"))
        })?;

        if file_type.is_dir() {
            files.extend(read_files_sync(&entry.path(), root_path)?);
        } else {
            let path = entry.path();
            let mut file = std::fs::File::open(&path)
                .map_err(|e| Error::unknown(format!("Failed to open file {path:?} ({e:?})")))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| Error::unknown(format!("Failed to read file {path:?} ({e:?})")))?;

            let path = path
                .strip_prefix(root_path)
                .map_err(|_| Error::unknown("Failed to strip path prefix".to_owned()))?;
            let path = path
                .to_str()
                .ok_or_else(|| Error::unknown("Failed to parse path to String".to_owned()))?
                .to_owned();
            files.push(CodeFile { path, content })
        }
    }

    Ok(files)
}
