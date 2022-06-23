use openapi_client::server::MakeService;

use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;

use rustserver::logger;
use rustserver::persistance::new_persistence;
use rustserver::service;
use rustserver::service::MySvc;

use rustserver::api::MyApi;

const DB_URI: &str = "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

/*
#[tokio::main]
async fn main() {
    logger::init();

    let pe = new_persistence(DB_URI).await;

    let svc = MySvc::new(pe);

    let (start, _stop) = service::serve(svc, "127.0.0.1:3000".to_string());

    start.start().await;
}*/

#[tokio::main]
async fn main() {
    logger::init();

    let pe = new_persistence(DB_URI).await;

    let api = MyApi::new(pe);

    let service = MakeService::new(api);
    let service = MakeAllowAllAuthenticator::new(service, "cosmo");
    let service = openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    let addr = "127.0.0.1:3000"
        .parse()
        .expect("Failed to parse bind address");

    hyper::server::Server::bind(&addr)
        .serve(service)
        .await
        .unwrap()
}
