use code_domain::model::{code_file::CodeFile, diagnostic_result::DocumentDiagnostics};
use common_domain::{define_repo, error::Result, tmp::TmpDirProvider};
use std::path::Path;
use std::path::PathBuf;

define_repo! {
    pub struct AnalyzeCodeRepository<B, C, D> {
        pub create_project: Fn<'a>(path: &'a Path) -> Result<PathBuf> as B,
        pub save_files: Fn<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()> as C,
        pub analyze: Fn<'a>(path: &'a Path) -> Result<Vec<DocumentDiagnostics>> as D,
    }
}

pub async fn analyze_code<A, B, C, D>(
    tmp_dir: A,
    repo: AnalyzeCodeRepository<B, C, D>,
    files: &[CodeFile],
) -> Result<Vec<DocumentDiagnostics>>
where
    A: TmpDirProvider,
    B: CreateProjectType,
    C: SaveFilesType,
    D: AnalyzeType,
{
    let path = tmp_dir.path();
    let files_path = (repo.create_project)(&path).await?;
    (repo.save_files)(&files_path, files).await?;

    (repo.analyze)(&path).await
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use code_domain::model::diagnostic_result::{Diagnostic, Position, Range, Severity};
    use common_domain::tmp::MockTmpDirProvider;

    use super::*;

    #[tokio::test]
    async fn analyze() {
        let mut mock_tmp = MockTmpDirProvider::new();
        let files = vec![CodeFile {
            path: "main.dart".to_owned(),
            content: "void main() {print('test')}".to_owned(),
        }];
        let document_diagnostics = vec![DocumentDiagnostics {
            path: "main.dart".to_owned(),
            diagnostics: vec![Diagnostic {
                code: "some_code".to_owned(),
                severity: Severity::WARNING,
                message: "some_message".to_owned(),
                range: Range {
                    start: Position { line: 1, column: 2 },
                    end: Position { line: 3, column: 4 },
                },
            }],
        }];
        let tmp_dir = temp_dir();
        let project_path = tmp_dir.join("project");

        let out = tmp_dir.clone();
        mock_tmp
            .expect_path()
            .times(1)
            .returning(move || out.clone());

        let (ctx, _create_project_lock) = mock_create_project::ctx().await;
        let out = project_path.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let (ctx, _save_files_lock) = mock_save_files::ctx().await;
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let (ctx, _analyze_lock) = mock_analyze::ctx().await;
        let out = document_diagnostics.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = analyze_code(
            mock_tmp,
            AnalyzeCodeRepository {
                create_project: mock_create_project::call,
                save_files: mock_save_files::call,
                analyze: mock_analyze::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(document_diagnostics));
    }
}
