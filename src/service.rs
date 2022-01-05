use std::sync::{Mutex, Arc};
use http::Request;
use http::Response;
use http::StatusCode;

use hyper::Body;
use super::persistance::Persistence;
use super::models::user::User;


fn new_id<'a>() -> &'a str {
    return "bla"
}

#[derive(Clone)]
pub struct MySvc {
     persistance: Box<Persistence>,
}


#[allow(dead_code)]
impl MySvc {
    
    pub async fn handle(self, req: Request<Body>) -> Result<Response<Body>, http::Error> {
        let resp = Response::builder().status(StatusCode::OK).body("".into()).expect("");
        

        let pe = self.persistance.clone();

        let user = pe.get_user_by_username("andre-0125cf21-418a-4b1c-8450-2d250e37f50b").await.unwrap();
        Ok(resp)
    }

    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, String> {
        let user = User::new(username, email, new_id());
        //let result = self.persistance.lock().unwrap().create_user(user).await;
        Ok(user)
        /*
        match result {
            Ok(usr) => Ok(usr),
            Err(err) => Err(err.to_string()),
        }
        */
    }
    
    pub fn new(persistance: Persistence) -> MySvc {
        MySvc{
            persistance: Box::new(persistance),
        }
    } 
}
