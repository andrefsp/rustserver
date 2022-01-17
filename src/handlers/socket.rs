use std::sync::Arc;

use async_trait::async_trait;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use http::Request;
use http::Response;
use http::StatusCode;

use hyper::upgrade::Upgraded;
use hyper::Body;

use super::super::persistance::DBPersistence;
use super::Handler;

#[derive(Clone)]
pub struct Socket {
    persistance: Arc<Box<dyn DBPersistence>>,
}

impl Socket {
    pub async fn ws_handle(self, mut upgraded: Upgraded) -> Result<(), std::io::Error> {
        loop {
            let mut payload = String::default();
            if let Err(err) = upgraded.read_to_string(&mut payload).await {
                return Err(err);
            }

            if let Err(err) = upgraded.write(payload.as_bytes()).await {
                return Err(err);
            }
        }
    }
}

#[async_trait]
impl Handler for Socket {
    type Target = Self;

    fn new(persistance: Arc<Box<dyn DBPersistence>>) -> Self::Target {
        Socket { persistance }
    }

    async fn handle(self, mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        if !req.headers().contains_key(hyper::header::UPGRADE) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("".into())
                .unwrap());
        }

        tokio::task::spawn(async move {
            match hyper::upgrade::on(&mut req).await {
                Ok(upgraded) => {
                    if let Err(err) = self.ws_handle(upgraded).await {
                        eprintln!("server foobar io error: {}", err)
                    };
                }
                Err(e) => eprintln!("upgrade error: {}", e),
            }
        });

        Ok(Response::builder()
            .status(StatusCode::SWITCHING_PROTOCOLS)
            .header("", "")
            .body("".into())
            .unwrap())
    }
}
