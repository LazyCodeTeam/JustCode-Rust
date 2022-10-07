use std::io::Cursor;

use crate::{
    model::error::DynError,
    util::{project::project_root, tmp_dir::TmpDir, zip::zip_file},
};

use super::CreateDartLayerArgs;

pub fn create_dart_layer(args: &CreateDartLayerArgs) -> Result<(), DynError> {
    let url = format!(
        "https://storage.googleapis.com/dart-archive/channels/stable/release/{}/sdk/dartsdk-linux-arm64-release.zip", 
        args.version,
    );

    let tmp = TmpDir::new()?;
    let tmp_dir = tmp.path();
    std::fs::create_dir_all(tmp_dir).ok();

    let dart_sdk_dir = tmp_dir.join("dart-sdk");
    std::fs::remove_dir_all(&dart_sdk_dir).ok();

    let zip_dir = tmp_dir.join("dart.zip");
    std::fs::remove_file(&zip_dir).ok();

    {
        let mut file = std::fs::File::create(&zip_dir)?;
        let response = reqwest::blocking::get(url)?;
        let mut content = Cursor::new(response.bytes()?);
        std::io::copy(&mut content, &mut file)?;
    }

    std::process::Command::new("unzip")
        .current_dir(tmp_dir)
        .arg(zip_dir)
        .status()?;

    let dart_bin_dir = dart_sdk_dir.join("bin").join("dart");

    let layer_dir = project_root().join("target").join("layer");
    std::fs::create_dir_all(&layer_dir).ok();
    let dart_layer_dir = layer_dir.join("dart.zip");
    std::fs::remove_file(&dart_layer_dir).ok();

    let path_in_zip = std::path::Path::new("bin").to_path_buf().join("dart");
    zip_file(&dart_bin_dir, &dart_layer_dir, &path_in_zip)?;

    Ok(())
}
