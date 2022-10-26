use code_domain::{
    model::code_file::CodeFile,
    port::{Build, CreateProject, SaveFiles},
};
use common_domain::{error::Result, port::Compress, tmp::TmpDirProvider};
use tokio::fs::File;

pub async fn build_code<A, B, C, D, E>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    build: D,
    compress: E,
    files: &[CodeFile],
) -> Result<File>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: Build<'a>,
    for<'a> E: Compress<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    let output_path = build(&path).await?;

    let output_zip = path.join("output.zip");

    compress(&output_path, &output_zip).await
}
