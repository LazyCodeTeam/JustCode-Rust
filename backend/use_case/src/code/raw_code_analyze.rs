use code_domain::model::{code_file::CodeFile, raw_message::RawMessage};
use common_domain::{define_repo, error::Result, tmp::TmpDirProvider};
use std::path::Path;
use std::path::PathBuf;

define_repo! {
    pub struct RawAnalyzeCodeRepository<B, C, D> {
        pub create_project: Fn<'a>(path: &'a Path) -> Result<PathBuf> as B,
        pub save_files: Fn<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()> as C,
        pub raw_analyze: FnOnce<'a>(path: &'a Path) -> Result<RawMessage> as D,
    }
}

pub async fn raw_code_analyze<A, B, C, D>(
    tmp_dir: A,
    repo: RawAnalyzeCodeRepository<B, C, D>,
    files: &[CodeFile],
) -> Result<RawMessage>
where
    A: TmpDirProvider,
    B: CreateProjectType,
    C: SaveFilesType,
    D: RawAnalyzeType,
{
    let path = tmp_dir.path();
    let files_path = (repo.create_project)(&path).await?;
    (repo.save_files)(&files_path, files).await?;

    (repo.raw_analyze)(&path).await
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use common_domain::tmp::MockTmpDirProvider;

    use super::*;

    #[tokio::test]
    async fn raw_analyze() {
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

        let (ctx, _create_project_lock) = mock_create_project::ctx().await;
        let out = project_path.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let (ctx, _save_files_lock) = mock_save_files::ctx().await;
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let (ctx, _raw_analyze_lock) = mock_raw_analyze::ctx().await;
        let out = message.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = raw_code_analyze(
            mock_tmp,
            RawAnalyzeCodeRepository {
                create_project: mock_create_project::call,
                save_files: mock_save_files::call,
                raw_analyze: mock_raw_analyze::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(message));
    }
}
