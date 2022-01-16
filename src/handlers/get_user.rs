use std::sync::Arc;

use http::Request;
use http::Response;
use http::StatusCode;

use async_trait::async_trait;

use hyper::Body;
use routerify::ext::RequestExt;

use super::super::persistance::DBPersistence;
use super::handlers::Handler;

#[derive(Clone)]
pub struct GetUser {
    persistance: Arc<Box<dyn DBPersistence>>,
}

#[async_trait]
impl Handler for GetUser {
    type Target = Self;

    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target {
        GetUser { persistance }
    }

    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let id = req.param("id").unwrap();
        let result = self.persistance.get_user_by_username(id).await;

        let resp = match result {
            Ok(user) => Response::builder()
                .status(StatusCode::OK)
                .body(user.to_json().into())
                .expect(""),
            Err(err) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(err.get_message().into())
                .expect(""),
        };
        Ok(resp)
    }
}
