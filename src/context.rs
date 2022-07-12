use super::persistance::DBPersistence;
use super::worker::WorkerTask;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Deps {
    pub persistance: Arc<Box<dyn DBPersistence>>,
    pub worker_tx: Sender<WorkerTask>,
}

impl Deps {
    pub fn new(persistance: Box<dyn DBPersistence>, worker_tx: Sender<WorkerTask>) -> Self {
        Self {
            worker_tx,
            persistance: Arc::new(persistance),
        }
    }
}
