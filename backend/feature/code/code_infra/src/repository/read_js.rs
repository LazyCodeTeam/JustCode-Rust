use std::path::Path;

use common_domain::error::{Error, Result};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn read_js(path: &Path) -> Result<String> {
    read_file(path)
        .await
        .map_err(|e| Error::unknown(format!("Failed to read file {path:?}: {e:?}")))
}

async fn read_file(path: &Path) -> std::result::Result<String, std::io::Error> {
    let mut file = File::open(path).await?;

    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    Ok(content)
}
