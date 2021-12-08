use super::persistance::DBPersistence;

pub struct Service<'s> {
    persistance: Box<dyn DBPersistence<'s>>,
}


pub fn new_service<'s>(persistance:  Box<dyn DBPersistence<'s>>) -> Service<'s> {
    Service{
        persistance,
    }
}
