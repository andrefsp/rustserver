use rustserver::persistance::new_persistence;
use rustserver::service::MySvc;
use rustserver::service;

use tokio::time::sleep;
use std::time::Duration;

const DB_URI: &str =  "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

#[tokio::main]
async fn main() {
    let pe = new_persistence(DB_URI).await;
    
    let svc = MySvc::new(pe);

    let (start, stop) = service::serve(svc, "127.0.0.1:3000".to_string());

    tokio::spawn(async {
        let timeout = Duration::new(15, 0);
        sleep(timeout).await;
        stop();
    });

    start.start().await;
}
