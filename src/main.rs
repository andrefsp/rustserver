use rustserver::context;
use rustserver::logger;
use rustserver::persistance::new_persistence;
use rustserver::service;
use rustserver::service::MySvc;
use rustserver::worker;
use rustserver::worker::WorkerTask;

const DB_URI: &str = "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

#[tokio::main]
async fn main() {
    logger::init();

    let pe = new_persistence(DB_URI).await;

    let hnd = Box::new(move |m: WorkerTask| {
        println!("payload: '{}'", m.payload);
    });

    let (mut p, tx, _worker_stop) = worker::new(hnd);

    let deps = context::Deps::new(pe, tx);

    let svc = MySvc::new(deps);

    let (start, _stop) = service::serve(svc, "127.0.0.1:3000".to_string());

    let svc_spawn = tokio::spawn(start.start());
    let worker_spawn = tokio::spawn(async move {
        p.start().await;
    });

    tokio::join!(svc_spawn, worker_spawn);
}
