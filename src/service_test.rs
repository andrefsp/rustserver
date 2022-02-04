use super::persistance::MockDBPersistence;
use super::service::MySvc;
use super::test::HttpTestServer;

#[tokio::test]
async fn test_service_response() {
    let persistance = MockDBPersistence::default();

    let svc = MySvc::new(Box::new(persistance));

    let result = HttpTestServer::new(svc).await;
    assert!(result.is_ok());

    let ts = result.unwrap();

    let client = hyper::Client::new();

    let uri = format!("{}", ts.url()).parse().unwrap();
    let response = client.get(uri).await;

    assert!(response.is_ok());

    let response = response.unwrap();

    assert_eq!(response.status(), http::StatusCode::OK);

    ts.shutdown()
}
