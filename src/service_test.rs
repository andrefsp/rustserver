use super::persistance::MockDBPersistence;
use super::service::MySvc;
use super::test::HttpTestServer;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::test]
async fn test_service_response() {
    let persistance = MockDBPersistence::default();
    let svc = MySvc::new(Box::new(persistance));

    let result = HttpTestServer::new(svc).await;
    assert!(result.is_ok());

    let ts = result.unwrap();

    let client = hyper::Client::new();

    let uri = format!("http://{}", ts.addr()).parse().unwrap();
    let response = client.get(uri).await;

    assert!(response.is_ok());

    let response = response.unwrap();

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    ts.shutdown()
}

#[tokio::test]
async fn test_ws_connect_and_echo() {
    let persistance = MockDBPersistence::default();
    let svc = MySvc::new(Box::new(persistance));

    let result = HttpTestServer::new(svc).await;
    assert!(result.is_ok());

    let ts = result.unwrap();

    let uri = format!("ws://{}/ws", ts.addr());

    let conn = connect_async(uri).await;
    assert!(conn.is_ok());

    let (mut ws, response) = conn.unwrap();
    assert_eq!(response.status(), http::StatusCode::SWITCHING_PROTOCOLS);

    let send = ws.send("hello".into()).await;
    assert!(send.is_ok());

    let m = ws.next().await.unwrap();
    assert!(m.is_ok());

    ts.shutdown()
}
