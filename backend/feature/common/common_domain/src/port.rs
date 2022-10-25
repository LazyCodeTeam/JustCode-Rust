use std::path::Path;

use crate::{define_port, error::Result};

define_port!(Compress = FnOnce<'a>(code: &'a Path, out: &'a Path) -> Result<tokio::fs::File>);
