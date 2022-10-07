use code_domain::{
    model::{code_file::CodeFile, raw_message::RawMessage},
    port::{CreateProject, RawAnalaze, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub async fn raw_code_analyze<A, B, C, D>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    raw_analyze: D,
    files: &[CodeFile],
) -> Result<RawMessage>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: RawAnalaze<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    raw_analyze(&path).await
}
