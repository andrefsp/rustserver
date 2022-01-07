use std::sync::Arc;
use http::Request;
use http::Response;
use http::StatusCode;

use tokio::sync::oneshot::{channel, Receiver};

use hyper::Body;
use hyper::server::Server;
use hyper::service::service_fn;

use tower::make::Shared;

use super::persistance::DBPersistence;
use super::models::user::User;

use uuid::Uuid;

fn new_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Clone)]
pub struct MySvc {
     persistance: Arc<Box<dyn DBPersistence>>,
}

#[allow(dead_code)]
impl MySvc {

    pub async fn handle(self, _req: Request<Body>) -> Result<Response<Body>, http::Error> {
        let resp = Response::builder().status(StatusCode::OK).body("".into()).expect("");

        let user = User::new("a", "b", "c");
        self.persistance.create_user(user).await.unwrap();

        //let user = pe.get_user_by_username("andre-0125cf21-418a-4b1c-8450-2d250e37f50b").await.unwrap();
        Ok(resp)
    }

    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, String> {
        let user = User::new(username, email, new_id().as_str());
        let result = self.persistance.create_user(user).await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(err.get_message().to_string()),
        }
    }

    pub fn new(persistance: Box<dyn DBPersistence>) -> MySvc {
        MySvc{
            persistance: Arc::new(persistance),
        }
    }
}


pub struct Executor {
    svc: MySvc,
    stop_rx: Receiver<()>,
    addr: String,
}

impl Executor {

    pub async fn start(self) {
        let svc = self.svc.clone();

        let make_service = Shared::new(service_fn(move |req| {
            svc.clone().handle(req)
        }));

        let addr = self.addr.as_str().parse().unwrap();

        let server = Server::bind(&addr)
            .serve(make_service)
            .with_graceful_shutdown(async {
                self.stop_rx.await.ok();
            });

        server.await.unwrap();
    }
}


pub fn serve(svc: MySvc, addr: String) -> (Executor, Box<dyn FnOnce() + Sync + Send>) {
    let (stop_tx, stop_rx) = channel();

    let stop_fn = move || {
        stop_tx.send(()).ok();
    };

    let stop_fn = Box::new(stop_fn);

    (Executor{
        svc,
        stop_rx,
        addr,
    }, stop_fn)
}

