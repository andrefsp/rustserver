use super::persistance::DBPersistence;
use super::models::user::User;

fn new_id<'a>() -> &'a str {
    return "bla"
}

pub struct MySvc {
     persistance: Box<dyn DBPersistence>,
}

#[allow(dead_code)]
impl MySvc {
    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, String> {
        let user = User::new(username, email, new_id());
        //let persistance = self.persistance.lock().unwrap();
        let result = self.persistance.create_user(user).await;

        match result {
            Ok(usr) => Ok(usr),
            Err(err) => Err(err.to_string()),
        }
    }
    
    pub fn new(persistance: Box<dyn DBPersistence>) -> MySvc {
        MySvc{
            persistance,
        }
    } 
}
