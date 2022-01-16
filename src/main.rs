use rustserver::persistance::new_persistence;
use rustserver::service;
use rustserver::service::MySvc;

use std::time::Duration;
use tokio::time::sleep;

const DB_URI: &str = "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

#[tokio::main]
async fn main() {
    let pe = new_persistence(DB_URI).await;

    let svc = MySvc::new(pe);

    let (start, _stop) = service::serve(svc, "127.0.0.1:3000".to_string());

    /*
    tokio::spawn(async {
        let timeout = Duration::new(15, 0);
        sleep(timeout).await;
        stop();
    });
    */

    start.start().await;
}
