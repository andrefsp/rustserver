use super::persistance::DBPersistence;
use super::models::user::User;
use std::sync::{Mutex, Arc};


fn new_id<'a>() -> &'a str {
    return "bla"
}

pub struct MySvc {
     persistance: Arc<Mutex<Box<dyn DBPersistence>>>,
}

#[allow(dead_code)]
impl MySvc {
    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, String> {
        let user = User::new(username, email, new_id());
        let result = self.persistance.lock().unwrap().create_user(user).await;

        match result {
            Ok(usr) => Ok(usr),
            Err(err) => Err(err.to_string()),
        }
    }
    
    pub fn new(persistance: Box<dyn DBPersistence>) -> MySvc {
        MySvc{
            persistance: Arc::new(Mutex::new(persistance)),
        }
    } 
}
