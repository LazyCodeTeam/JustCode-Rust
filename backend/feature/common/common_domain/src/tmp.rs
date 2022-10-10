use std::path::PathBuf;

use mockall::automock;

#[automock]
pub trait TmpDirProvider {
    fn path(&self) -> PathBuf;
}
