use code_domain::{
    model::code_file::CodeFile,
    port::{Build, CreateProject, SaveFiles},
};
use common_domain::{error::Result, port::Compress, tmp::TmpDirProvider};
use tokio::fs::File;

pub async fn build_code<A, B, C, D, E>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    build: D,
    compress: E,
    files: &[CodeFile],
) -> Result<File>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: Build<'a>,
    for<'a> E: Compress<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    let output_path = build(&path).await?;

    let output_zip = path.join("output.zip");

    compress(&output_path, &output_zip).await
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

        let _compress_lock = common_domain::port::compress_lock().await;
        let ctx = common_domain::port::mock_compress::call_context();
        let test_file_path = temp_dir().join("test_file");
        let test_file = tokio::fs::File::create(&test_file_path).await.unwrap();
        let out = test_file.try_clone().await.unwrap();

        ctx.expect().times(1).return_once(move |_, __| Ok(out));

        let result = build_code(
            mock_tmp,
            code_domain::port::mock_create_project::call,
            code_domain::port::mock_save_files::call,
            code_domain::port::mock_build::call,
            common_domain::port::mock_compress::call,
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
