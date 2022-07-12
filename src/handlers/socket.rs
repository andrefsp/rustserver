use async_trait::async_trait;
use futures::{sink::SinkExt, stream::StreamExt};

use http::Request;
use http::Response;
use http::StatusCode;

use hyper::Body;

use hyper_tungstenite::{tungstenite, HyperWebsocket};
use tungstenite::Message;

use super::super::context::Deps;
use super::Handler;

#[derive(Clone)]
pub struct Socket {
    deps: Deps,
}

impl Socket {
    pub fn new(deps: Deps) -> Self {
        Socket { deps }
    }

    pub async fn ws_handle(self, ws: HyperWebsocket) -> Result<(), tungstenite::Error> {
        let mut ws = ws.await?;

        loop {
            let message = ws.next().await;
            if message.is_none() {
                return Ok(());
            }

            let message = message.unwrap()?;

            let echo_msg = format!("> '{}'", message);
            ws.send(Message::text(echo_msg)).await?;
        }
    }
}

#[async_trait]
impl Handler for Socket {
    async fn handle(self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        if !hyper_tungstenite::is_upgrade_request(&req) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("".into())
                .unwrap());
        }

        let upgrade = hyper_tungstenite::upgrade(req, None);
        if let Err(err) = upgrade {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{}", err).into())
                .unwrap());
        }

        let (resp, ws) = upgrade.unwrap();

        tokio::spawn(async move { self.ws_handle(ws).await });
        Ok(resp)
    }
}
