use code_domain::{
    model::{code_file::CodeFile, raw_message::RawMessage},
    port::{CreateProject, RawAnalyze, SaveFiles},
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
    for<'a> D: RawAnalyze<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    raw_analyze(&path).await
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use common_domain::tmp::MockTmpDirProvider;

    use super::*;

    #[tokio::test]
    async fn format() {
        let mut mock_tmp = MockTmpDirProvider::new();
        let files = vec![CodeFile {
            path: "main.dart".to_owned(),
            content: "void main() {print('test');}".to_owned(),
        }];
        let message = RawMessage {
            success: false,
            message: "some_message".to_owned(),
        };
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

        let _raw_analyze_lock = code_domain::port::raw_analyze_lock().await;
        let ctx = code_domain::port::mock_raw_analyze::call_context();
        let out = message.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = raw_code_analyze(
            mock_tmp,
            code_domain::port::mock_create_project::call,
            code_domain::port::mock_save_files::call,
            code_domain::port::mock_raw_analyze::call,
            &files,
        )
        .await;

        assert_eq!(result, Ok(message));
    }
}
