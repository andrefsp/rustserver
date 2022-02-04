use super::service::serve;
use super::service::MySvc;
use tokio::net::TcpStream;

pub struct HttpTestServer {
    addr: String,
    stop: Box<dyn FnOnce() + Sync + Send>,
}

impl HttpTestServer {
    pub fn shutdown(self) {
        (self.stop)();
    }

    fn pick_addr() -> String {
        "127.0.0.1:4000".into()
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub async fn new(svc: MySvc) -> Result<Self, std::io::Error> {
        let addr = HttpTestServer::pick_addr();

        let (exec, stop) = serve(svc, addr.clone());

        let test_server = HttpTestServer { addr, stop };

        tokio::spawn(exec.start());

        for _ in 0..10 {
            let stm = TcpStream::connect(test_server.addr.clone()).await;
            match stm {
                Ok(_) => return Ok(test_server),
                _ => continue,
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "an error has occured",
        ))
    }
}
