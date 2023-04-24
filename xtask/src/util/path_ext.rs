use sha3::Digest;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};
use zip::write::FileOptions;

pub trait PathExt: AsRef<Path> {
    fn project_root_path() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1) // Because returned path is for xtask
            .expect("Failed to find project root")
            .to_path_buf()
    }

    fn is_dir(&self) -> bool {
        std::fs::metadata(self)
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
    }

    fn is_file(&self) -> bool {
        std::fs::metadata(self)
            .map(|meta| meta.is_file())
            .unwrap_or(false)
    }

    fn is_same_as(&self, other: &Path) -> bool {
        match (self.is_dir(), other.is_dir()) {
            (a, b) if a == b => self.hashed() == other.hashed(),
            _ => false,
        }
    }

    fn hashed(&self) -> Vec<u8> {
        if self.is_file() {
            let mut file = std::fs::File::open(self).unwrap();
            let mut hasher = sha3::Sha3_256::new();
            std::io::copy(&mut file, &mut hasher).unwrap();

            hasher.finalize().to_vec()
        } else {
            self.as_ref()
                .to_str()
                .expect("Failed to convert path to string")
                .as_bytes()
                .to_vec()
        }
    }

    fn zip_file_to(&self, target: &Path, zipped_name: &str) {
        if !self.is_file() {
            panic!("Path is not a file");
        }
        let mut source_file = std::fs::File::open(self).expect("Failed to open source file");

        let file = std::fs::File::create(target).expect("Failed to create target file");
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file(zipped_name, FileOptions::default())
            .expect("Failed to start file");
        let metadata = std::fs::metadata(self).expect("Failed to get metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        source_file
            .read_exact(&mut buffer)
            .expect("Failed to read file");
        zip.write_all(&buffer).expect("Failed to write file");
        zip.finish().expect("Failed to finish file");
    }
}

impl PathExt for PathBuf {}
impl PathExt for Path {}

#[cfg(test)]
mod test {
    use super::*;

    const CARGO_FILE: &str = "Cargo.toml";

    #[test]
    fn test_is_dir() {
        let path = Path::project_root_path();
        assert!(path.is_dir());
    }

    #[test]
    fn test_is_not_dir() {
        let path = Path::project_root_path().join(CARGO_FILE);
        assert!(!path.is_dir());
    }

    #[test]
    fn test_project_root_path() {
        let path = Path::project_root_path();
        assert!(path.is_dir());
    }

    #[test]
    fn test_is_file() {
        let path = Path::project_root_path().join(CARGO_FILE);
        assert!(path.is_file());
    }

    #[test]
    fn test_is_not_file() {
        let path = Path::project_root_path();
        assert!(!path.is_file());
    }

    #[test]
    fn test_is_same_as_dirs() {
        let path = Path::project_root_path();
        let path2 = Path::project_root_path();
        assert!(path.is_same_as(&path2));
    }

    #[test]
    fn test_dir_is_not_same_as_file() {
        let path = Path::project_root_path();
        let path2 = Path::project_root_path().join(CARGO_FILE);
        assert!(!path.is_same_as(&path2));
    }

    #[test]
    fn test_file_is_not_same_as_dir() {
        let path = Path::project_root_path().join(CARGO_FILE);
        let path2 = Path::project_root_path();
        assert!(!path.is_same_as(&path2));
    }

    #[test]
    fn test_is_same_as_same_files() {
        let path = Path::project_root_path().join(CARGO_FILE);
        let path2 = Path::project_root_path().join(CARGO_FILE);
        assert!(path.is_same_as(&path2));
    }

    #[test]
    fn test_is_not_same_as_different_files() {
        let path = Path::project_root_path().join(CARGO_FILE);
        let path2 = Path::project_root_path().join("Cargo.lock");
        assert!(!path.is_same_as(&path2));
    }

    #[test]
    fn test_is_not_same_as_different_dirs() {
        let path = Path::project_root_path().join("xtask");
        let path2 = Path::project_root_path();
        assert!(!path.is_same_as(&path2));
    }
}
