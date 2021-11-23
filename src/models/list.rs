use super::items::ItemType;

#[allow(dead_code)]
pub struct List<'a> {
    id: i32,
    user_id: i32,
    name: &'a str,
    items: Vec<ItemType>,
}

#[allow(dead_code)]
impl <'a> List<'a> {
    pub fn new(id: i32, name: &'a str, user_id: i32) -> List<'a> {
        List{
            id,
            name,
            user_id,
            items: Vec::new(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn add_item(&mut self, item: ItemType) {
        self.items.push(item);
    }
}
