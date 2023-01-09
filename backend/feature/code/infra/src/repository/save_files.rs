use std::path::Path;

use code_domain::model::code_file::CodeFile;
use common_domain::error::{Error, Result};
use tokio::io::AsyncWriteExt;

pub async fn save_files(path: &Path, files: &[CodeFile]) -> Result<()> {
    for file in files.iter() {
        let file_path = path.join(&file.path);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }

        let mut f = tokio::fs::File::create(file_path)
            .await
            .map_err(|e| Error::unknown(e.to_string()))?;

        f.write_all(file.content.as_bytes())
            .await
            .map_err(|e| Error::unknown(e.to_string()))?;
    }

    Ok(())
}
