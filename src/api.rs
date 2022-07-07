use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use futures::future::FutureExt;

use serde_json;

use async_trait::async_trait;

use hyper::service::Service;
use hyper::{Body, HeaderMap, Request, Response, StatusCode};

use swagger::auth::RcBound;
use swagger::auth::{Authorization, Scopes};
use swagger::ApiError;
use swagger::{Has, XSpanIdString};

use openapi::Api;

use openapi::models::CreateUserRequest;
use openapi::CreateUserResponse;

use super::persistance::DBPersistence;

pub struct MakeMyAuth<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    inner: T,
    persistance: Arc<Box<dyn DBPersistence>>,

    marker: PhantomData<RC>,
}

impl<T, RC> MakeMyAuth<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    pub fn new(inner: T, persistance: Arc<Box<dyn DBPersistence>>) -> Self {
        MakeMyAuth {
            inner,
            persistance,
            marker: PhantomData,
        }
    }
}

impl<Inner, RC, Target> Service<Target> for MakeMyAuth<Inner, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Inner: Service<Target>,
    Inner::Future: Send + 'static,
{
    type Error = Inner::Error;
    type Response = MyAuth<Inner::Response, RC>;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let pe = self.persistance.clone();
        Box::pin(self.inner.call(target).map(|s| Ok(MyAuth::new(s?, pe))))
    }
}

#[derive(Clone)]
pub struct MyAuth<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    inner: T,
    persistance: Arc<Box<dyn DBPersistence>>,

    marker: PhantomData<RC>,
}

impl<T, RC> MyAuth<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    pub fn new(inner: T, persistance: Arc<Box<dyn DBPersistence>>) -> Self {
        MyAuth {
            inner,
            persistance,
            marker: PhantomData,
        }
    }
}

impl<T, B, RC> Service<(Request<B>, RC)> for MyAuth<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    T: Service<(Request<B>, RC::Result)>,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<B>, RC)) -> Self::Future {
        let (request, context) = req;
        let context = context.push(Some(Authorization {
            subject: "".into(),
            scopes: Scopes::Some(BTreeSet::from([
                "write:users".to_string(),
                "read:users".to_string(),
            ])),
            issuer: None,
        }));

        self.inner.call((request, context))
    }
}

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
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync,
{
    /// Add a new user to the store
    async fn create_user(
        &self,
        request: Option<CreateUserRequest>,
        context: &C,
    ) -> Result<CreateUserResponse, ApiError> {
        let user = self
            .persistance
            .get_user_by_id("a51d74ab-b7f4-4618-bc72-ae949b2d3f64")
            .await
            .unwrap();

        let userp = serde_json::to_value(user);

        Ok(CreateUserResponse::SuccessfulOperation(userp.unwrap()))
    }
}
