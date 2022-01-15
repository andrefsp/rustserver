use std::sync::Arc;

use http::Request;
use http::Response;

use async_trait::async_trait;

use hyper::Body;

use super::super::persistance::DBPersistence;


#[async_trait]
pub trait Handler: Clone {
    type Target;

    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error>;
    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target;
}
