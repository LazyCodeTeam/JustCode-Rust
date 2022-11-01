use code_domain::{
    model::{code_file::CodeFile, diagnostic_result::DocumentDiagnostics},
    port::{Analyze, CreateProject, SaveFiles},
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
    for<'a> D: Analyze<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    analyze(&path).await
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

        let _create_project_lock = code_domain::port::create_project_lock().await;
        let ctx = code_domain::port::mock_create_project::call_context();
        let out = project_path.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let _save_files_lock = code_domain::port::save_files_lock().await;
        let ctx = code_domain::port::mock_save_files::call_context();
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let _analyze_lock = code_domain::port::analyze_lock().await;
        let ctx = code_domain::port::mock_analyze::call_context();
        let out = document_diagnostics.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = analyze_code(
            mock_tmp,
            code_domain::port::mock_create_project::call,
            code_domain::port::mock_save_files::call,
            code_domain::port::mock_analyze::call,
            &files,
        )
        .await;

        assert_eq!(result, Ok(document_diagnostics));
    }
}
