use hyper::server::Server;
use rustserver::persistance::new_persistence;
use rustserver::service::MySvc;

use hyper::{Body, Request, Response};
use hyper::service::service_fn;

use tower::make::Shared;

const DB_URI: &str =  "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

#[tokio::main]
async fn main() {
    let pe = new_persistence(DB_URI).await;
    
    let svc = MySvc::new(pe);
 
    let make_service = Shared::new(service_fn(move |req| {
        svc.clone().handle(req)
    }));


    let addr = "127.0.0.1:3000".parse().unwrap();

    let server = Server::bind(&addr).serve(make_service);

    server.await.unwrap();
}
