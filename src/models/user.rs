use std::fmt::Display; 
use std::result::Result;
use std::fmt::Error;
use std::fmt::Formatter;
use std::cmp::{PartialEq, PartialOrd, Ord, Eq, Ordering};

pub struct User<'a> {
    username: &'a str,
    email: &'a str,
    id: i32,
}

impl Display for User<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt, "email: {}, username: {}, id: {}", 
            self.get_email(), self.get_username(), self.get_id()
        )
    }
}

impl PartialOrd for User<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for User<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
         if self.id < other.id {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialEq for User<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.username == other.username
    }
}

impl Eq for User<'_> {}


impl<'a> User<'a> {
    pub fn new(username: &'a str, email: &'a str, id: i32) -> User<'a> {
        User{
            username,
            email,
            id,
        }
    }

    pub fn get_username(&self) -> &str {
        self.username
    }
    
    pub fn get_email(&self) -> &str {
        self.email
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
