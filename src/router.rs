use hyper::{Body, Request, Response};
use routerify::Router;
use super::service::Svc;
use std::future::Future;
//use futures::Future;

fn home_handler(svc: Svc, _req: Request<Body>) -> impl Future<Output = Result<Response<Body>, hyper::Error>> {
    async move {
        svc.create_user("user1", "email@email.com").await.unwrap();

        Ok(Response::new(Body::from("Home page")))
    }
}


pub fn new(svc: Svc) -> Router<Body, hyper::Error> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    //
    let h_svc = svc.clone();
    let x_svc = svc.clone();
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .get("/", move |req| { 
            home_handler(h_svc.clone(), req)
        })
        .get("/x/", move |req| { 
            home_handler(x_svc.clone(), req)
        })
        .build()
        .unwrap()
}
