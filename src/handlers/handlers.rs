use http::Request;
use http::Response;

use async_trait::async_trait;

use hyper::Body;

#[async_trait]
pub trait Handler {
    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error>;
}
