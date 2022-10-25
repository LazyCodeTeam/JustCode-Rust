use std::{io::Write, path::Path};

use common_domain::error::{Error, Result};
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use zip::write::FileOptions;

pub async fn compress(input: &Path, output: &Path) -> Result<tokio::fs::File> {
    tokio::fs::create_dir_all(output.parent().unwrap_or(output))
        .await
        .ok();
    let metadata = tokio::fs::metadata(input)
        .await
        .map_err(|e| Error::unknown(format!("Failed to read meatadata of {input:?}: {e:?}")))?;

    if metadata.is_dir() {
        compress_dir(input, output)
            .await
            .map_err(|e| Error::unknown(format!("Failed to compress directory {input:?}: {e:?}")))
    } else {
        compress_file(input, output)
            .await
            .map_err(|e| Error::unknown(format!("Failed to compress file {input:?}: {e:?}")))
    }
}

pub async fn compress_file(
    input: &Path,
    output: &Path,
) -> std::result::Result<tokio::fs::File, std::io::Error> {
    let mut file_in = tokio::fs::File::open(input).await?;
    let file = std::fs::File::create(output)?;
    let mut zip = zip::ZipWriter::new(file);
    let name_in_zip = input
        .components()
        .last()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to get last component")
        })?
        .as_os_str()
        .to_str()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get parse path to string",
            )
        })?;
    zip.start_file(name_in_zip, FileOptions::default())?;
    let metadata = tokio::fs::metadata(input).await?;
    let mut buffer = vec![0; metadata.len() as usize];
    file_in.read_exact(&mut buffer).await?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    tokio::fs::File::open(output).await
}

pub async fn compress_dir(
    input: &Path,
    output: &Path,
) -> std::result::Result<tokio::fs::File, std::io::Error> {
    let file = std::fs::File::create(output)?;
    let mut zip = zip::ZipWriter::new(file);
    let files = WalkDir::new(input).into_iter().filter_map(|r| r.ok());
    for entry in files {
        let path = entry.path();
        let zip_path = path
            .strip_prefix(input)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to strip prefix"))?
            .to_str()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to get parse path to string",
                )
            })?;
        if path.is_file() {
            zip.start_file(zip_path, FileOptions::default())?;
            let mut f = tokio::fs::File::open(path).await?;
            let metadata = tokio::fs::metadata(path).await?;
            let mut buffer = vec![0; metadata.len() as usize];

            f.read_exact(&mut buffer).await?;
            zip.write_all(&buffer)?;
        } else if !path.as_os_str().is_empty() {
            zip.add_directory(zip_path, FileOptions::default())?;
        }
    }

    zip.finish()?;
    tokio::fs::File::open(output).await
}
