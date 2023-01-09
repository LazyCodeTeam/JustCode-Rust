use code_domain::{
    model::code_file::CodeFile,
    port::{CreateProject, FormatFiles, ReadFiles, SaveFiles},
};
use common_domain::{error::Result, tmp::TmpDirProvider};

pub struct FormatCodeRepository<B, C, D, E> {
    pub create_project: B,
    pub save_files: C,
    pub format_files: D,
    pub read_files: E,
}

pub async fn format_code<A, B, C, D, E>(
    tmp_dir: A,
    repo: FormatCodeRepository<B, C, D, E>,
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

        let _create_project_lock = code_domain::port::create_project_lock().await;
        let ctx = code_domain::port::mock_create_project::call_context();
        let out = project_path.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let _save_files_lock = code_domain::port::save_files_lock().await;
        let ctx = code_domain::port::mock_save_files::call_context();
        ctx.expect().times(1).returning(|_, _| Ok(()));

        let _format_files_lock = code_domain::port::format_files_lock().await;
        let ctx = code_domain::port::mock_format_files::call_context();
        ctx.expect().times(1).returning(|_| Ok(()));

        let _read_files_lock = code_domain::port::read_files_lock().await;
        let ctx = code_domain::port::mock_read_files::call_context();
        let out = files_out.clone();
        ctx.expect().times(1).returning(move |_| Ok(out.clone()));

        let result = format_code(
            mock_tmp,
            FormatCodeRepository {
                create_project: code_domain::port::mock_create_project::call,
                save_files: code_domain::port::mock_save_files::call,
                format_files: code_domain::port::mock_format_files::call,
                read_files: code_domain::port::mock_read_files::call,
            },
            &files,
        )
        .await;

        assert_eq!(result, Ok(files_out))
    }
}
