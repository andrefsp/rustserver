use super::persistance::new_persistence;
use super::models::user::User;
use sqlx::MySqlPool;

const DB_URI: &str =  "mysql://root@localhost:3306/testdb?parseTime=true&charset=utf8mb4";

async fn get_db_pool() -> MySqlPool {
    let pool = MySqlPool::connect(DB_URI).await;
    pool.unwrap()
}

#[tokio::test]
async fn test_persistence() {
    let pool = get_db_pool().await;
    let p = new_persistence(pool);
    let user = p.get_user_by_id(10);
    assert_eq!(user.get_id(), 10);
}

#[tokio::test]
async fn test_persistence_create_and_get_user() {
    let pool = get_db_pool().await;
    let p = new_persistence(pool);

    let user = User::new("andre", "email@email.com", 10);

    let result = p.create_user(user).await;

    assert!(result.is_ok());
    
    //let err = result.unwrap_err();
    //assert_eq!(err.get_message(), "persistence_error");
}

#[tokio::test]
async fn test_persistence_can_connect() {     
    let pool = get_db_pool();
    
    let p = new_persistence(pool.await);

    //sqlx::query("SELECT * from users").fetch_all()
}
