use code_domain::{
    model::code_file::CodeFile,
    port::{CreateProject, FormatFiles, ReadFiles, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub async fn format_code<A, B, C, D, E>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    format_files: D,
    read_files: E,
    files: &[CodeFile],
) -> Result<Vec<CodeFile>>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: FormatFiles<'a>,
    for<'a> E: ReadFiles<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    format_files(&path).await?;

    read_files(&files_path).await
}
