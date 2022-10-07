use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::model::error::DynError;

pub fn zip_file(
    file_path: &PathBuf,
    output_file: &PathBuf,
    path_in_zip: &Path,
) -> Result<(), DynError> {
    let file_out = std::fs::File::create(output_file)?;
    let mut file_in = std::fs::File::open(file_path)?;

    let mut zip = zip::ZipWriter::new(file_out);

    zip.start_file(path_in_zip.to_str().unwrap(), FileOptions::default())?;
    let mut buffer = Vec::new();
    file_in.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;

    zip.finish()?;

    Ok(())
}

pub fn zip_dir(
    dir: &PathBuf,
    output_file: &PathBuf,
    path_prefix: Option<&PathBuf>,
) -> Result<(), DynError> {
    let file = std::fs::File::create(output_file)?;

    let walkdir = WalkDir::new(dir);
    let it = walkdir.into_iter().filter_map(|entry| entry.ok());
    let mut buffer = Vec::new();
    let mut zip = zip::ZipWriter::new(file);

    for entry in it {
        let path = entry.path();
        let mut name = path.strip_prefix(std::path::Path::new(dir))?.to_owned();

        if let Some(p) = path_prefix {
            name = p.join(name);
        }

        if path.is_file() {
            let mut file_in = std::fs::File::open(path)?;
            zip.start_file(name.to_str().unwrap(), FileOptions::default())?;

            file_in.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;

            buffer.clear();
        }
    }

    zip.finish()?;
    Ok(())
}
