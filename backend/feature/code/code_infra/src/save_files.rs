use std::{fs::File, io::Write, path::Path};

use code_domain::model::code_file::CodeFile;
use common_domain::error::{Error, Result};

pub async fn save_files(path: &Path, files: &[CodeFile]) -> Result<()> {
    for file in files.iter() {
        let file_path = path.join(&file.path);
        let mut f = File::create(file_path).map_err(|e| Error::unknown(&e.to_string()))?;

        f.write_all(file.content.as_bytes())
            .map_err(|e| Error::unknown(&e.to_string()))?;
    }

    Ok(())
}
