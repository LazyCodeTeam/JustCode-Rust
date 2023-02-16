use code_domain::model::code_file::CodeFile;
use common_domain::{define_repo, error::Result, tmp::TmpDirProvider};
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;

define_repo! {
    pub struct BuildCodeRepository<B, C, D, E> {
        pub create_project: Fn<'a>(path: &'a Path) -> Result<PathBuf> as B,
        pub save_files: Fn<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()> as C,
        pub build: Fn<'a>(path: &'a Path) -> Result<PathBuf> as D,
        pub compress: Fn<'a>(code: &'a Path, out: &'a Path) -> Result<tokio::fs::File> as E,
    }
}

pub async fn build_code<A, B, C, D, E>(
    tmp_dir: A,
    repo: BuildCodeRepository<B, C, D, E>,
    files: &[CodeFile],
) -> Result<File>
where
    A: TmpDirProvider,
    B: CreateProjectType,
    C: SaveFilesType,
    D: BuildType,
    E: CompressType,
{
    let path = tmp_dir.path();
    let files_path = (repo.create_project)(&path).await?;
    (repo.save_files)(&files_path, files).await?;

    let output_path = (repo.build)(&path).await?;

    let output_zip = path.join("output.zip");

    (repo.compress)(&output_path, &output_zip).await
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use common_domain::tmp::MockTmpDirProvider;

    use super::*;

    #[tokio::test]
    async fn build() {
        let mut mock_tmp = MockTmpDirProvider::new();
        let files = vec![CodeFile {
            path: "main.dart".to_owned(),
            content: "void main() {print('test');}".to_owned(),
        }];
        let tmp_dir = temp_dir();
        let out_dir = temp_dir().join("out");
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

        let (ctx, _build_lock) = mock_build::ctx().await;
        let out = out_dir.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let (ctx, _compress_lock) = mock_compress::ctx().await;
        let test_file_path = temp_dir().join("test_file");
        let test_file = tokio::fs::File::create(&test_file_path).await.unwrap();
        let out = test_file.try_clone().await.unwrap();

        ctx.expect().times(1).return_once(move |_, __| Ok(out));

        let result = build_code(
            mock_tmp,
            BuildCodeRepository {
                create_project: mock_create_project::call,
                save_files: mock_save_files::call,
                build: mock_build::call,
                compress: mock_compress::call,
            },
            &files,
        )
        .await
        .unwrap();

        let result_metadata = result.metadata().await.unwrap();
        let test_file_metadata = test_file.metadata().await.unwrap();

        assert_eq!(
            result_metadata.modified().unwrap(),
            test_file_metadata.modified().unwrap()
        );

        tokio::fs::remove_file(test_file_path).await.unwrap();
    }
}
