use code_domain::{
    model::code_file::CodeFile,
    port::{Build, CreateProject, ReadJs, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub struct BuildCode2jsRepository<B, C, D, E> {
    pub create_project: B,
    pub save_files: C,
    pub build: D,
    pub read_js: E,
}

pub async fn build_code_2js<A, B, C, D, E>(
    tmp_dir: A,
    repo: BuildCode2jsRepository<B, C, D, E>,
    files: &[CodeFile],
) -> Result<String>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: Build<'a>,
    for<'a> E: ReadJs<'a>,
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

        let _create_project_lock = code_domain::port::create_project_lock().await;
        let ctx = code_domain::port::mock_create_project::call_context();
        let out = project_path.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let _save_files_lock = code_domain::port::save_files_lock().await;
        let ctx = code_domain::port::mock_save_files::call_context();
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let _build_lock = code_domain::port::build_lock().await;
        let ctx = code_domain::port::mock_build::call_context();
        let out = out_dir.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let _read_js_lock = code_domain::port::read_js_lock().await;
        let ctx = code_domain::port::mock_read_js::call_context();
        let out = js_content.clone();
        ctx.expect().times(1).return_once(move |_| Ok(out));

        let result = build_code_2js(
            mock_tmp,
            BuildCode2jsRepository {
                create_project: code_domain::port::mock_create_project::call,
                save_files: code_domain::port::mock_save_files::call,
                build: code_domain::port::mock_build::call,
                read_js: code_domain::port::mock_read_js::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(js_content))
    }
}
