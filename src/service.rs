use std::sync::Arc;

use tokio::sync::oneshot::{channel, Receiver};

use hyper::server::Server;
use hyper::Body;

use routerify::Router;
use routerify::RouterService;

use super::handlers::{CreateUser, GetUser, Handler, Socket};
use super::persistance::DBPersistence;

#[derive(Clone)]
pub struct MySvc {
    persistance: Arc<Box<dyn DBPersistence>>,
}

#[allow(dead_code)]
impl MySvc {
    pub fn router(&self) -> Router<Body, hyper::Error> {
        // Create the handlers here
        let get_user_hnd = GetUser::new(self.persistance.clone());
        let create_user_hnd = CreateUser::new(self.persistance.clone());
        let socket = Socket::new(self.persistance.clone());

        // hook handlers with appropriate URI
        Router::builder()
            .get("/users/:id", move |req| get_user_hnd.clone().handle(req))
            .post("/users/", move |req| create_user_hnd.clone().handle(req))
            .any_method("/ws", move |req| socket.clone().handle(req))
            .build()
            .unwrap()
    }

    pub fn new(persistance: Box<dyn DBPersistence>) -> MySvc {
        MySvc {
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
        let addr = self.addr.as_str().parse().unwrap();
        let service = RouterService::new(self.svc.router()).unwrap();

        let server = Server::bind(&addr)
            .serve(service)
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

    (Executor { svc, stop_rx, addr }, stop_fn)
}
