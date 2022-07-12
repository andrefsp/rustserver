use std::convert::From;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot::{channel as oneshotchannel, Receiver as OneShotReceiver};

type HndFn<T> = Box<dyn Fn(T) + Send + Sync>;

type StopFn = Box<dyn FnOnce() + Sync + Send>;

pub struct WorkerTask {
    pub payload: String,
}

impl From<&str> for WorkerTask {
    fn from(msg: &str) -> Self {
        Self {
            payload: msg.to_string(),
        }
    }
}

pub struct Worker<T> {
    rx: Receiver<T>,
    hnd: HndFn<T>,

    stop_rx: OneShotReceiver<()>,
}

impl<T> Worker<T> {
    pub fn create(rx: Receiver<T>, stop_rx: OneShotReceiver<()>, hnd: HndFn<T>) -> Self {
        Self { rx, hnd, stop_rx }
    }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                Some(message) = self.rx.recv() => {
                    (self.hnd)(message)
                }
                _ = &mut self.stop_rx => {
                    self.rx.close();
                    self.stop_rx.close();
                    return
                }
            }
        }
    }
}

pub fn new<T>(hnd: HndFn<T>) -> (Worker<T>, Sender<T>, StopFn) {
    let (tx, rx) = channel::<T>(10);

    let (stop_tx, stop_rx) = oneshotchannel();

    let stop = move || {
        stop_tx.send(()).unwrap();
    };
    let worker = Worker::create(rx, stop_rx, hnd);
    (worker, tx, Box::new(stop))
}
