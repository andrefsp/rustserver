use std::sync::Arc;

use http::Request;

use super::super::models::user::User;
use super::Handler;

#[tokio::test]
async fn test_create_user_uses_persistence() {
    use super::super::persistance::MockDBPersistence;

    let mut persistance = MockDBPersistence::default();

    persistance
        .expect_create_user()
        .times(1)
        .returning(|user| Ok(user));

    let p = Box::new(persistance);
    let hnd = super::CreateUser::new(Arc::new(p));

    let user = User::new("username", "email@email.com", "someid");

    let req = Request::builder().body(user.to_json().into()).unwrap();
    let result = hnd.handle(req).await;

    assert!(result.is_ok());
}
