use std::path::{Path, PathBuf};

use common_domain::{define_port, error::Result};

use crate::model::{
    code_file::CodeFile, diagnostic_result::DocumentDiagnostics, raw_message::RawMessage,
};

define_port!(CreateProject = FnOnce<'a>(path: &'a Path) -> Result<PathBuf>);

define_port!(RawAnalyze = FnOnce<'a>(path: &'a Path) -> Result<RawMessage>);

define_port!(Analyze = FnOnce<'a>(path: &'a Path) -> Result<Vec<DocumentDiagnostics>>);

define_port!(SaveFiles = FnOnce<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()>);

define_port!(ReadFiles = FnOnce<'a>(path: &'a Path) -> Result<Vec<CodeFile>>);

define_port!(FormatFiles = FnOnce<'a>(path: &'a Path) -> Result<()>);

define_port!(GetVersion = FnOnce() -> Result<String>);
