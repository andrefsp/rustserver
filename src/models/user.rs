use std::fmt::Display; 
use std::result::Result;
use std::fmt::Error;
use std::fmt::Formatter;
use std::cmp::{PartialEq, PartialOrd, Ord, Eq, Ordering};
use sqlx::FromRow;

#[derive(FromRow, Clone)]
pub struct User {
    username: String,
    email: String,
    id: String,
}


impl Display for User {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt, "email: {}, username: {}, id: {}", 
            self.get_email(), self.get_username(), self.get_id()
        )
    }
}


impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
         if self.id < other.id {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.username == other.username
    }
}

impl Eq for User {}


impl<'a> User {
    pub fn new(username: &'a str, email: &'a str, id: &'a str) -> User {
        User{
            username: String::from(username),
            email: String::from(email),
            id: String::from(id),
        }
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }
    
    pub fn get_email(&self) -> &str {
        self.email.as_str()
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}
