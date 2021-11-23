use std::fmt::Display;
use std::error::Error;
use std::fmt::{Formatter, Error as FmtError};
use sqlx::MySqlPool;
use sqlx::query;
use async_trait::async_trait;

use super::models::user::User;
use super::models::list::List;
use super::models::items::ItemType;

#[derive(Debug)]
pub struct PersistenceError<'a> {
    message: &'a str,
}

impl<'a> PersistenceError<'_> {
    pub fn new(message: &'a str) -> PersistenceError<'a> {
        PersistenceError{
            message: message.clone(),
        }
    }

    pub fn get_message(&self) -> &str {
        self.message
    }
}

impl Display for PersistenceError<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(fmt, "err: {}", self.message) 
    } 
}

impl Error for PersistenceError<'_> {}

// Database persistance trait
#[async_trait]
pub trait DBPersistence<'a> {
    async fn create_user<'u>(&self, user: User<'u>) -> Result<(), PersistenceError>;
    fn get_user_by_id(&self, id: i32) -> User<'_>; 
    fn get_user_by_username(&self, _username: &'a str) -> User<'_>; 
    fn get_user_lists(&self, _user_id: i32) -> Vec<List<'_>>; 
    fn get_list_items(&self, _list_id: i32) -> Vec<ItemType> ;
    fn migrate(&self) -> Result<(), PersistenceError>;
}


#[allow(dead_code)]
pub struct Persistence {
    pool: MySqlPool,
}

#[allow(dead_code)]
#[async_trait]
impl<'a> DBPersistence<'a> for Persistence {
    async fn create_user<'u>(&self, user: User<'u>) -> Result<(), PersistenceError> {
        let result = sqlx::query(
            format!("INSERT INTO users(id, username, email) VALUES({}, '{}', '{}')", 
                user.get_id(),
                user.get_username(),
                user.get_email()
            ).as_str()
        ).execute(&self.pool).await;
        
        match result {
            Err(err) => {
                println!("err>> {}", err);
                Err(PersistenceError::new("error!"))
            },
            Ok(_) => {Ok(())},
        }

    }

    fn get_user_by_id(&self, id: i32) -> User<'_> {
        User::new("andrefsp", "email@email.com", id)
    }

    fn get_user_by_username(&self, _username: &'a str) -> User<'_> {
        User::new("", "email@email.com", 1)
    }

    fn get_user_lists(&self, _user_id: i32) -> Vec<List<'_>> {
        Vec::new()
    }

    fn get_list_items(&self, _list_id: i32) -> Vec<ItemType> {
        Vec::new()
    }

    fn migrate(&self) -> Result<(), PersistenceError> {
        Ok(())
    }
}

#[allow(dead_code)]
pub fn new_persistence<'a>(pool: MySqlPool) -> Box<dyn DBPersistence<'a>> {
    Box::new(Persistence{
        pool,
    })
}
