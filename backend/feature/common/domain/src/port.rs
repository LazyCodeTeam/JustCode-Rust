use std::path::Path;

use crate::{define_port, error::Result};

use crate as common_domain;

define_port!(Compress = FnOnce<'a>(code: &'a Path, out: &'a Path) -> Result<tokio::fs::File>);
