use code_domain::model::code_file::CodeFile;
use common_domain::{define_repo, error::Result, tmp::TmpDirProvider};
use std::path::{Path, PathBuf};

define_repo! {
    pub struct BuildCode2jsRepository<B, C, D, E> {
        pub create_project: Fn<'a>(path: &'a Path) -> Result<PathBuf> as B,
        pub save_files: Fn<'a>(path: &'a Path, files: &'a[CodeFile]) -> Result<()> as C,
        pub build: Fn<'a>(path: &'a Path) -> Result<PathBuf> as D,
        pub read_js: Fn<'a>(path: &'a Path) -> Result<String> as E,
    }
}
pub async fn build_code_2js<A, B, C, D, E>(
    tmp_dir: A,
    repo: BuildCode2jsRepository<B, C, D, E>,
    files: &[CodeFile],
) -> Result<String>
where
    A: TmpDirProvider,
    B: CreateProjectType,
    C: SaveFilesType,
    D: BuildType,
    E: ReadJsType,
{
    let path = tmp_dir.path();
    let files_path = (repo.create_project)(&path).await?;
    (repo.save_files)(&files_path, files).await?;

    let output_path = (repo.build)(&path).await?;

    (repo.read_js)(&output_path).await
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use common_domain::tmp::MockTmpDirProvider;

    use super::*;

    #[tokio::test]
    async fn build_2js() {
        let mut mock_tmp = MockTmpDirProvider::new();
        let files = vec![CodeFile {
            path: "main.dart".to_owned(),
            content: "void main() {print('test');}".to_owned(),
        }];
        let tmp_dir = temp_dir();
        let out_dir = temp_dir().join("out");
        let project_path = tmp_dir.join("project");
        let js_content = "main() {console.log('test')}".to_owned();

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

        let (ctx, _read_js_lock) = mock_read_js::ctx().await;
        let out = js_content.clone();
        ctx.expect().times(1).return_once(move |_| Ok(out));

        let result = build_code_2js(
            mock_tmp,
            BuildCode2jsRepository {
                create_project: mock_create_project::call,
                save_files: mock_save_files::call,
                build: mock_build::call,
                read_js: mock_read_js::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(js_content))
    }
}
