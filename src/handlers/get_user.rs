use http::Request;
use http::Response;
use http::StatusCode;

use async_trait::async_trait;

use hyper::Body;
use routerify::ext::RequestExt;

use super::super::context::Deps;
use super::handlers::Handler;

#[derive(Clone)]
pub struct GetUser {
    deps: Deps,
}

impl GetUser {
    pub fn new(deps: Deps) -> Self {
        GetUser { deps }
    }
}

#[async_trait]
impl Handler for GetUser {
    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let id = req.param("id").unwrap();
        let result = self.deps.persistance.get_user_by_username(id).await;

        if let Err(_) = self.deps.worker_tx.send("this the payload".into()).await {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .expect(""));
        };

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
