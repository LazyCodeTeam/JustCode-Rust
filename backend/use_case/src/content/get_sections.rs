use common_domain::{define_repo, error::Error, error::Result};
use content_domain::model::section::Section;

define_repo! {
    pub struct GetSectionsRepo<A> {
        pub get_sections: Fn<'a>(technology_id: &'a str) -> Result<Vec<Section>> as A,
    }
}

pub async fn get_sections<A>(
    technology_id: String,
    repo: GetSectionsRepo<A>,
) -> Result<Vec<Section>>
where
    A: GetSectionsType,
{
    let sections = (repo.get_sections)(&technology_id).await?;

    if sections.is_empty() {
        return Err(Error::not_found());
    }

    Ok(sections)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn no_sections() {
        let (ctx, _get_sections_lock) = mock_get_sections::ctx().await;
        ctx.expect()
            .once()
            .withf(|technology_id| technology_id == "technology_id")
            .returning(|_| Ok(vec![]));

        let repo = GetSectionsRepo {
            get_sections: mock_get_sections::call,
        };

        let result = get_sections("technology_id".to_owned(), repo).await;

        assert_eq!(result, Err(Error::not_found()));
    }

    #[tokio::test]
    async fn sections() {
        let sections = vec![Section::default(), Section::default()];
        let (ctx, _get_sections_lock) = mock_get_sections::ctx().await;
        let out = sections.clone();
        ctx.expect()
            .once()
            .withf(|technology_id| technology_id == "technology_id")
            .return_once(move |_| Ok(out));

        let repo = GetSectionsRepo {
            get_sections: mock_get_sections::call,
        };

        let result = get_sections("technology_id".to_owned(), repo).await;

        assert_eq!(result, Ok(sections));
    }
}
