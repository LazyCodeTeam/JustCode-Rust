use serde::Deserialize;
use std::path::{Path, PathBuf};

use super::path_ext::PathExt;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct CargoContent {
    pub package: Package,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Package {
    pub name: String,
}

#[derive(Debug)]
pub struct Project {
    crates: Vec<Crate>,
}

#[derive(Debug)]
pub struct Crate {
    cargo: CargoContent,
    is_bin: bool,
}

impl CargoContent {
    pub fn read_from(path: &Path) -> Self {
        let content = std::fs::read_to_string(path).expect("Failed to read Cargo.toml");
        toml::from_str(&content).unwrap()
    }
}

impl Crate {
    pub fn new(path: PathBuf) -> Self {
        let cargo = CargoContent::read_from(&path.join("Cargo.toml"));
        let is_bin = path.join("src").join("main.rs").is_file();

        Self { cargo, is_bin }
    }
}

impl Project {
    pub fn new() -> Self {
        let root = Path::project_root_path();
        let crates = Self::get_crates(&root);

        Self { crates }
    }

    fn get_crates(path: &Path) -> Vec<Crate> {
        if !path.is_dir() {
            return vec![];
        }
        if path.to_path_buf().ends_with("target") {
            return vec![];
        }

        let cargo_path = path.join("Cargo.toml");

        if cargo_path.is_file() && path != Path::project_root_path().as_path() {
            return vec![Crate::new(path.to_path_buf())];
        }

        let mut crates = vec![];

        for dir in std::fs::read_dir(path).expect("Failed to read directory") {
            let dir = dir.expect("Failed to read directory entry");
            let dir = dir.path();

            if dir.is_dir() {
                crates.append(&mut Self::get_crates(&dir));
            }
        }

        crates
    }

    pub fn bin_crates_names(&self) -> Vec<&str> {
        self.crates
            .iter()
            .filter(|c| c.is_bin)
            .map(|c| c.cargo.package.name.as_str())
            .filter(|n| !n.contains("xtask"))
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_get_bin_names() {
        let project = Project::new();
        let bin_names = project.bin_crates_names();

        assert!(!bin_names.is_empty());
    }
}
