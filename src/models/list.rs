use super::items::ItemType;
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(FromRow)]
pub struct List {
    id: String,
    user_id: String,
    name: String,
    items: Vec<ItemType>,
}

#[allow(dead_code)]
impl List {
    pub fn new(id: &str, name: &str, user_id: &str) -> List {
        List {
            id: String::from(id),
            name: String::from(name),
            user_id: String::from(user_id),
            items: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub fn add_item(&mut self, item: ItemType) {
        self.items.push(item);
    }
}
