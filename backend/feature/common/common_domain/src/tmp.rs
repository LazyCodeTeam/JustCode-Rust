use std::path::PathBuf;

pub trait TmpDirProvider {
    fn path(&self) -> PathBuf;
}
