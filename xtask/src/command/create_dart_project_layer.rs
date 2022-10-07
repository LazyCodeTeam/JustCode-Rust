use std::path::Path;

use crate::{
    model::error::{DynError, Error},
    util::{project::project_root, tmp_dir::TmpDir, zip::zip_dir},
};

use super::CreateDartProjectLayerArgs;

pub fn create_dart_project_layer(args: &CreateDartProjectLayerArgs) -> Result<(), DynError> {
    let tmp_dir = TmpDir::new()?;
    let dart_path = project_root()
        .join("tmp")
        .join("flutter")
        .join("bin")
        .join("dart");

    if !dart_path.exists() {
        return Err(Box::new(Error::DartDoesNotExist));
    }

    let pub_cache_dir = tmp_dir.path().join("pub_cache");
    std::env::set_var("PUB_CACHE", pub_cache_dir);

    let status = std::process::Command::new(&dart_path)
        .current_dir(tmp_dir.path())
        .args(vec!["create", "project"])
        .status()?;

    if !status.success() {
        return Err(Box::new(Error::FailedToCreateDartProject));
    }

    let project_dir = tmp_dir.path().join("project");

    for package in &args.packages {
        let status = std::process::Command::new(&dart_path)
            .current_dir(&project_dir)
            .args(vec!["pub", "add", package])
            .status()?;

        if !status.success() {
            return Err(Box::new(Error::FailedToAddDartPackage(package.to_owned())));
        }
    }

    let layer_path = project_root().join("target").join("layer");
    std::fs::create_dir_all(&layer_path).ok();

    let zip_path = layer_path.join("dart_project.zip");
    std::fs::remove_dir_all(&zip_path).ok();
    zip_dir(
        tmp_dir.path(),
        &zip_path,
        Some(&Path::new("lib").to_path_buf()),
    )?;

    Ok(())
}
