use super::models::user::User;
use super::persistance::new_persistence;
use uuid::Uuid;

const DB_URI: &str = "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

fn new_test_user() -> User {
    let id = Uuid::new_v4().to_hyphenated().to_string();
    User::new(
        format!("andre-{}", id).as_str(),
        format!("email-{}@email.com", &id).as_str(),
        &id,
    )
}

#[tokio::test]
async fn test_persistence_create_and_get_user() {
    let p = new_persistence(DB_URI).await;
    let user = new_test_user();

    let result = p.create_user(user).await;
    assert!(result.is_ok());
    let user = result.unwrap();

    let result = p.get_user_by_id(user.get_id()).await;
    assert!(result.is_ok());
    let db_user = result.unwrap();
    assert_eq!(user.get_id(), db_user.get_id());
    assert_eq!(user.get_email(), db_user.get_email());
    assert_eq!(user.get_username(), db_user.get_username());

    let result = p.get_user_by_username(user.get_username()).await;
    assert!(result.is_ok());
    let db_user = result.unwrap();
    assert_eq!(user.get_id(), db_user.get_id());
    assert_eq!(user.get_email(), db_user.get_email());
    assert_eq!(user.get_username(), db_user.get_username());
}
