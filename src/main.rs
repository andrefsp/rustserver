use rustserver::logger;
use rustserver::persistance::new_persistence;
use rustserver::service;
use rustserver::service::MySvc;

const DB_URI: &str = "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

#[tokio::main]
async fn main() {
    logger::init();

    let pe = new_persistence(DB_URI).await;

    let svc = MySvc::new(pe);

    let (start, _stop) = service::serve(svc, "127.0.0.1:3000".to_string());

    start.start().await;
}
