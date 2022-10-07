use std::path::{Path, PathBuf};

use common_domain::{define_port, error::Result};

use crate::model::{code_file::CodeFile, raw_message::RawMessage};

define_port!(CreateProject = FnOnce<'a>(&'a PathBuf) -> Result<PathBuf>);

define_port!(RawAnalaze = FnOnce<'a>(&'a PathBuf) -> Result<RawMessage>);

define_port!(SaveFiles = FnOnce<'a>(&'a Path, &'a[CodeFile]) -> Result<()>);
