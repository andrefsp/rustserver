use super::worker;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot::channel as oneshot_channel;
use tokio::sync::oneshot::Sender;

#[tokio::test]
async fn test_worker_handler_receives_messages() {
    let messages = Vec::<String>::new();
    let messages = Arc::new(Mutex::new(messages));

    let hnd_messages = messages.clone();
    let hnd = Box::new(move |s: String| {
        let mut messages = hnd_messages.lock().unwrap();
        messages.push(s);
    });

    let (mut p, tx, stop) = worker::new(hnd);

    tokio::spawn(async move {
        p.start().await;
    });

    assert!(tx.send("this".to_string()).await.is_ok());
    assert!(tx.send("this".to_string()).await.is_ok());

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    assert_eq!(messages.lock().unwrap().len(), 2);

    stop();
}

#[tokio::test]
async fn test_worker_bounces_with_channel() {
    struct Message {
        payload: String,
        pub tx_complete: Sender<()>,
    }

    let hnd = Box::new(move |m: Message| {
        println!("{}", m.payload);
        m.tx_complete.send(()).unwrap();
    });

    let (mut p, tx, stop) = worker::new(hnd);

    tokio::spawn(async move {
        p.start().await;
    });

    let (transaction_tx, transaction_rx) = oneshot_channel::<()>();

    let message = Message {
        payload: String::from("message"),
        tx_complete: transaction_tx,
    };

    assert!(tx.send(message).await.is_ok());

    tokio::select! {
        _ = transaction_rx => {}
    };

    stop();
}
