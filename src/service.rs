use std::sync::Arc;

use tokio::sync::oneshot::{channel, Receiver};

use hyper::server::Server;
use hyper::Body;

use routerify::Router;
use routerify::RouterService;

use super::handlers::{CreateUser, GetUser, Handler, Socket};
use super::persistance::DBPersistence;

// router macro avoids repeating loads of boilerplate code related with
// cloning an handler and wiring it with the router methods
macro_rules! router {
    (
        $(
            ($method:expr, $path:expr, $hnd:expr),
        )
        *
    ) => {{
        let r = Router::builder();

        $(
            let r = match $method {
                "GET" => r.get($path, move |req| $hnd.clone().handle(req)),
                "POST" => r.post($path, move |req| $hnd.clone().handle(req)),
                "DELETE" => r.delete($path, move |req| $hnd.clone().handle(req)),
                "PUT" => r.put($path, move |req| $hnd.clone().handle(req)),
                "PATCH" => r.patch($path, move |req| $hnd.clone().handle(req)),
                "OPTIONS" => r.options($path, move |req| $hnd.clone().handle(req)),
                "TRACE" => r.trace($path, move |req| $hnd.clone().handle(req)),
                _ => r.any_method($path, move |req| $hnd.clone().handle(req)),
            };
        )*

        r.build().unwrap()
    }};
}

#[derive(Clone)]
pub struct MySvc {
    persistance: Arc<Box<dyn DBPersistence>>,
}

#[allow(dead_code)]
impl MySvc {
    pub fn router(&self) -> Router<Body, hyper::Error> {
        // Create the handlers here
        let get_user = GetUser::new(self.persistance.clone());
        let create_user = CreateUser::new(self.persistance.clone());
        let socket = Socket::new(self.persistance.clone());

        router!(
            ("GET", "/users/:id", get_user),
            ("POST", "/users", create_user),
            ("*", "/ws", socket),
        )
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
