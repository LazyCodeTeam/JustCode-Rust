use code_domain::{
    model::{code_file::CodeFile, diagnostic_result::DocumentDiagnostics},
    port::{Analaze, CreateProject, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub async fn analyze_code<A, B, C, D>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    analyze: D,
    files: &[CodeFile],
) -> Result<Vec<DocumentDiagnostics>>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: Analaze<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    analyze(&path).await
}
