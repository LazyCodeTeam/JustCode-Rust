use std::{fs::create_dir_all, path::Path};

use crate::util::{path_ext::PathExt, project::Project};

use super::BuildLambdasArgs;

pub fn build_lambdas(args: &BuildLambdasArgs) {
    run_cargo_build(&args.target, args.use_cross);
    let project = Project::new();
    let zip_target = Path::project_root_path().join("target").join("lambdas");
    let bins_path = Path::project_root_path()
        .join("target")
        .join(&args.target)
        .join("release");
    create_dir_all(&zip_target).ok();

    for lambda in project.bin_crates_names() {
        zip_lambda(lambda, &bins_path, &zip_target, &args.entrypoint);
    }
}

fn zip_lambda(name: &str, bins_path: &Path, zip_target: &Path, zipped_name: &str) {
    let bin_path = bins_path.join(name);
    let bin_target_path = zip_target.join(name);

    if bin_path.is_same_as(&bin_target_path) {
        return;
    }

    let zip_path = zip_target.join(format!("{name}.zip"));
    bin_path.zip_file_to(&zip_path, zipped_name);

    std::fs::copy(bin_path, zip_target.join(name)).expect("Failed to copy lambda");
}

fn run_cargo_build(target: &str, use_cross: bool) {
    let cargo = if use_cross {
        "cross".to_owned()
    } else {
        std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
    };
    let status = std::process::Command::new(cargo)
        .current_dir(Path::project_root_path())
        .args([
            "build",
            "--release",
            "--target",
            target,
            "--exclude",
            "xtask",
            "--workspace",
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        panic!("Failed to build project");
    }
}
