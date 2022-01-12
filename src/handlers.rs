use std::sync::Arc;

use uuid::Uuid;

use http::Request;
use http::Response;
use http::StatusCode;

use async_trait::async_trait;

use hyper::Body;

use routerify::ext::RequestExt;

use super::persistance::DBPersistence;
use super::models::user::User;


fn new_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}


#[async_trait]
pub trait Handler: Clone {
    type Target;

    async fn handle(self,_req: Request<Body>) -> Result<Response<Body>, hyper::Error>;
    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target;
}


#[derive(Clone)]
pub struct CreateUser {    
     persistance: Arc<Box<dyn DBPersistence>>,
}

#[async_trait]
impl Handler for CreateUser {
    type Target = Self;

    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target {
        CreateUser{
            persistance
        }
    }

    async fn handle(self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let user = User::new("username", "email", new_id().as_str());
        self.persistance.create_user(user).await.unwrap();

        let resp = Response::builder()
            .status(StatusCode::OK)
            .body("".into())
            .expect("");

        Ok(resp)
    }
}

#[derive(Clone)]
pub struct GetUser{    
     persistance: Arc<Box<dyn DBPersistence>>,
}

#[async_trait]
impl Handler for GetUser {
    type Target = Self;

    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target {
        GetUser{
            persistance
        }
    }

    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let id = req.param("id").unwrap();

        let _user = self.persistance.get_user_by_username(id).await.unwrap();

        let resp = Response::builder()
            .status(StatusCode::OK)
            .body("".into())
            .expect("");

        Ok(resp)
    }
}
