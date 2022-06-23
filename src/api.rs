use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;

use swagger::ApiError;
use swagger::{Has, XSpanIdString};

use openapi_client::models::User;
use openapi_client::Api;

use openapi_client::CreateUserResponse;

use super::persistance::DBPersistence;

#[derive(Clone)]
pub struct MyApi<C> {
    persistance: Arc<Box<dyn DBPersistence>>,
    marker: PhantomData<C>,
}

impl<C> MyApi<C> {
    pub fn new(persistance: Box<dyn DBPersistence>) -> Self {
        MyApi {
            persistance: Arc::new(persistance),
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<C> Api<C> for MyApi<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Add a new user to the store
    async fn create_user(
        &self,
        user: Option<User>,
        context: &C,
    ) -> Result<CreateUserResponse, ApiError> {
        let user = self
            .persistance
            .get_user_by_id("a51d74ab-b7f4-4618-bc72-ae949b2d3f64")
            .await
            .unwrap();

        Ok(CreateUserResponse::SuccessfulOperation(User {
            id: Some(String::from(user.get_id())),
            email: Some("".to_string()),
            username: Some("".to_string()),
        }))
    }
}
