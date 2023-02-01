use common_domain::error::Result;
use profile_domain::{
    model::push_data::PushData,
    port::{RemovePushData, UpdatePushData},
};

pub struct SetPushDataRepository<A, B> {
    pub update_push_data: A,
    pub remove_push_data: B,
}

pub async fn set_push_data<A, B>(
    (id, data): (String, Option<PushData>),
    repo: SetPushDataRepository<A, B>,
) -> Result<()>
where
    for<'a> A: UpdatePushData<'a>,
    for<'a> B: RemovePushData<'a>,
{
    match data {
        Some(data) => (repo.update_push_data)(&id, &data).await,
        None => (repo.remove_push_data)(&id).await,
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;
    use profile_domain::model::platform::Platform;

    use super::*;

    #[tokio::test]
    async fn updates_data() {
        let id = "123";
        let push_data = PushData {
            token: "token".to_owned(),
            platform: Platform::Android,
        };
        let _update_push_data_lock = profile_domain::port::update_push_data_lock();
        let ctx = profile_domain::port::mock_update_push_data::call_context();
        ctx.expect()
            .with(eq(id.to_owned()), eq(push_data.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let _remove_push_data_lock = profile_domain::port::remove_push_data_lock();
        let ctx = profile_domain::port::mock_remove_push_data::call_context();
        ctx.expect().times(0);

        let repo = SetPushDataRepository {
            update_push_data: profile_domain::port::mock_update_push_data::call,
            remove_push_data: profile_domain::port::mock_remove_push_data::call,
        };

        let result = set_push_data((id.to_owned(), Some(push_data)), repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn removes_data() {
        let id = "123";
        let _update_push_data_lock = profile_domain::port::update_push_data_lock();
        let ctx = profile_domain::port::mock_update_push_data::call_context();
        ctx.expect().times(0);

        let _remove_push_data_lock = profile_domain::port::remove_push_data_lock();
        let ctx = profile_domain::port::mock_remove_push_data::call_context();
        ctx.expect()
            .with(eq(id.to_owned()))
            .times(1)
            .returning(|_| Ok(()));

        let repo = SetPushDataRepository {
            update_push_data: profile_domain::port::mock_update_push_data::call,
            remove_push_data: profile_domain::port::mock_remove_push_data::call,
        };

        let result = set_push_data((id.to_owned(), None), repo).await;

        assert!(result.is_ok());
    }
}
