use hyper::{Body, Request};
use routerify::prelude::RequestExt;
use routerify::Middleware;

// A middleware which logs an http request.
pub async fn logger(req: Request<Body>) -> Result<Request<Body>, hyper::Error> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}
