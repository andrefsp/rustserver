use std::str;

use uuid::Uuid;

use http::Request;
use http::Response;
use http::StatusCode;

use async_trait::async_trait;

use hyper::Body;

use super::super::context::Deps;
use super::super::models::user::User;

use super::handlers::Handler;

fn new_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Clone)]
pub struct CreateUser {
    deps: Deps,
}

impl CreateUser {
    pub fn new(deps: Deps) -> Self {
        CreateUser { deps }
    }
}

#[async_trait]
impl Handler for CreateUser {
    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let body = req.into_body();
        let bytes = hyper::body::to_bytes(body).await?;
        let payload = str::from_utf8(&bytes).unwrap().to_string();

        let object = User::from_json(payload);

        if let Err(err) = object {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("{}", err).into())
                .expect(""));
        }

        let user = object.unwrap();

        let resp = match self.deps.persistance.create_user(user).await {
            Ok(user) => Response::builder()
                .status(StatusCode::OK)
                .body(user.to_json().into())
                .expect(""),
            Err(err) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("{}", err).into())
                .expect(""),
        };

        Ok(resp)
    }
}
