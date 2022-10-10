use code_domain::{
    model::code_file::CodeFile,
    port::{CreateProject, FormatFiles, ReadFiles, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub async fn format_code<A, B, C, D, E>(
    tmp_dir: A,
    create_project: B,
    save_files: C,
    format_files: D,
    read_files: E,
    files: &[CodeFile],
) -> Result<Vec<CodeFile>>
where
    A: TmpDirProvider,
    for<'a> B: CreateProject<'a>,
    for<'a> C: SaveFiles<'a>,
    for<'a> D: FormatFiles<'a>,
    for<'a> E: ReadFiles<'a>,
{
    let path = tmp_dir.path();
    let files_path = create_project(&path).await?;
    save_files(&files_path, files).await?;

    format_files(&path).await?;

    read_files(&files_path).await
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

        let out = project_path.clone();
        let ctx = code_domain::port::mock_create_project::call_context();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let ctx = code_domain::port::mock_save_files::call_context();
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let ctx = code_domain::port::mock_format_files::call_context();
        ctx.expect().times(1).returning(|_| Ok(()));

        let ctx = code_domain::port::mock_read_files::call_context();
        let out = files_out.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = format_code(
            mock_tmp,
            code_domain::port::mock_create_project::call,
            code_domain::port::mock_save_files::call,
            code_domain::port::mock_format_files::call,
            code_domain::port::mock_read_files::call,
            &files,
        )
        .await;

        assert_eq!(result, Ok(files_out))
    }
}
