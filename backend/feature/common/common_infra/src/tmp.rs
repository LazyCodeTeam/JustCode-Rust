use common_domain::{
    error::{Error, Result},
    tmp::TmpDirProvider,
};
use nanoid::nanoid;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TmpDir {
    path: PathBuf,
}

impl TmpDir {
    pub fn new() -> Result<TmpDir> {
        let name = nanoid!();
        let dir = std::env::temp_dir().join("tessar").join(name);
        std::fs::create_dir_all(&dir)
            .map_err(|e| Error::unknown(&format!("Failed to create tmp dir {dir:?} - {e:?}")))?;
        Ok(TmpDir { path: dir })
    }
}

impl TmpDirProvider for TmpDir {
    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).ok();
    }
}
