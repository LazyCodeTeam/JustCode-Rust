use code_domain::model::code_file::CodeFile;
use common_domain::{define_repo, error::Result, tmp::TmpDirProvider};
use std::path::Path;
use std::path::PathBuf;

define_repo! {
    pub struct FormatCodeRepository<B, C, D, E> {
        pub create_project: Fn<'a>(path: &'a Path) -> Result<PathBuf> as B,
        pub save_files: Fn<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()> as C,
        pub format_files: Fn<'a>(path: &'a Path) -> Result<()> as D,
        pub read_files: Fn<'a>(path: &'a Path) -> Result<Vec<CodeFile>> as E,
    }
}

pub async fn format_code<A, B, C, D, E>(
    tmp_dir: A,
    repo: FormatCodeRepository<B, C, D, E>,
    files: &[CodeFile],
) -> Result<Vec<CodeFile>>
where
    A: TmpDirProvider,
    B: CreateProjectType,
    C: SaveFilesType,
    D: FormatFilesType,
    E: ReadFilesType,
{
    let path = tmp_dir.path();
    let files_path = (repo.create_project)(&path).await?;
    (repo.save_files)(&files_path, files).await?;

    (repo.format_files)(&path).await?;

    (repo.read_files)(&files_path).await
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
        let files_out = vec![CodeFile {
            path: "main.dart".to_owned(),
            content: "void main() {\n  print('test');\n}\n".to_owned(),
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

        let (ctx, _format_files_lock) = mock_format_files::ctx().await;
        ctx.expect().times(1).returning(|_| Ok(()));

        let (ctx, _read_files_lock) = mock_read_files::ctx().await;
        let out = files_out.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = format_code(
            mock_tmp,
            FormatCodeRepository {
                create_project: mock_create_project::call,
                save_files: mock_save_files::call,
                format_files: mock_format_files::call,
                read_files: mock_read_files::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(files_out))
    }
}
