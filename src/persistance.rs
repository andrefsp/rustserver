use std::fmt::Display;
use std::error::Error;
use std::fmt::{Formatter, Error as FmtError};


use sqlx::MySqlPool;
use async_trait::async_trait;

use mockall::automock;

use super::models::user::User;

#[derive(Debug)]
pub struct PersistenceError {
    message: String,
}

#[allow(dead_code)]
impl PersistenceError {
    pub fn new(message: &str) -> PersistenceError {
        PersistenceError{
            message: String::from(message),
        }
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

impl Display for PersistenceError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(fmt, "err: {}", self.message)
    }
}

impl Error for PersistenceError {}

// Database persistance trait
#[automock]
#[async_trait]
pub trait DBPersistence: Sync + Send {
    async fn create_user(&self, user: User) -> Result<User, PersistenceError>;
    async fn get_user_by_id<'u>(&self, id: &'u str) -> Result<User, PersistenceError>;
    async fn get_user_by_username<'u>(&self, _username: &'u str) -> Result<User, PersistenceError>;
    async fn migrate(&self) -> Result<(), PersistenceError>;
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct Persistence {
    pool: MySqlPool,
}

#[allow(dead_code)]
impl Persistence {
    async fn get_user_by_key(&self, key: &str, val: &str) ->  Result<User, PersistenceError> {
        let query = match key {
            "username" => Some("SELECT * FROM users WHERE username = ?"),
            "id" => Some("SELECT * FROM users WHERE id = ?"),
            _ => None,
        };

        match query {
            None => panic!("Expected key with value... got {}", key),
            Some(text) => {
                let result = sqlx::query_as::<_, User>(text)
                    .bind(val)
                    .fetch_one(&self.pool).await;

                match result {
                    Err(err) =>
                        Err(PersistenceError::new(err.to_string().as_str())),
                    Ok(row) => Ok(row),
                }
            }
        }
    }
}

#[async_trait]
impl DBPersistence for Persistence {
    async fn create_user(&self, user: User) -> Result<User, PersistenceError> {
        let result = sqlx::query("
            INSERT INTO users(id, username, email) VALUES(?, ?, ?)
        ")
            .bind(user.get_id())
            .bind(user.get_username())
            .bind(user.get_email())
            .execute(&self.pool)
            .await;

        match result {
            Err(err) =>
                Err(PersistenceError::new(err.to_string().as_str())),
            Ok(_) => Ok(user),
        }
    }

    async fn get_user_by_id<'u>(&self, id: &'u str) -> Result<User, PersistenceError> {
        self.get_user_by_key("id", id).await
    }

    async fn get_user_by_username<'u>(&self, username: &'u str) -> Result<User, PersistenceError> {
        self.get_user_by_key("username", username).await
    }

    async fn migrate(&self) -> Result<(), PersistenceError> {
        Ok(())
    }
}

#[allow(dead_code)]
async fn get_db_pool(uri: &str) -> MySqlPool {
    let pool = MySqlPool::connect(uri).await;
    pool.unwrap()
}

#[allow(dead_code)]
pub async fn new_persistence(uri: &str) -> Box<dyn DBPersistence> {
    Box::new(Persistence{
        pool: get_db_pool(uri).await,
    })
}
