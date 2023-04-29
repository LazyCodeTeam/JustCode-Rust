use common_domain::{
    define_repo,
    error::Result,
    error::{Error, ResultLogExt},
};
use content_domain::model::section::Section;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct GetPublicSectionsRepo<A> {
        pub get_sections: Fn<'a>(technology_id: &'a str) -> Result<Vec<Section>> as A,
    }
}

#[derive(Debug, Snafu)]
pub enum GetPublicSectionsError {
    #[snafu(display("Sections for technology {} not found", technology_id))]
    NotFound {
        technology_id: String,
    },
    Infra {
        source: Error,
    },
}

pub async fn get_public_sections<A>(
    technology_id: String,
    repo: GetPublicSectionsRepo<A>,
) -> std::result::Result<Vec<Section>, GetPublicSectionsError>
where
    A: GetSectionsType,
{
    let sections = (repo.get_sections)(&technology_id)
        .await
        .context(InfraSnafu)?;

    if sections.is_empty() {
        return Err(GetPublicSectionsError::NotFound { technology_id }).with_debug_log();
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

        let repo = GetPublicSectionsRepo {
            get_sections: mock_get_sections::call,
        };

        let result = get_public_sections("technology_id".to_owned(), repo).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GetPublicSectionsError::NotFound { technology_id } if technology_id == "technology_id"
        ));
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

        let repo = GetPublicSectionsRepo {
            get_sections: mock_get_sections::call,
        };

        let result = get_public_sections("technology_id".to_owned(), repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sections);
    }
}
