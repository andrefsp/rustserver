#[allow(dead_code)]
pub enum ItemType {
    Meeting{
        at: i32,
        address: String,
    },
    Task {
        at: i32,
        description: String,
        epic: String
    },
}

pub fn new_meeting(at: i32, address: String) -> ItemType {
    ItemType::Meeting{
        at,
        address,
    }
}

pub fn new_task(at: i32, description: String, epic: String) -> ItemType {
    ItemType::Task{
        at,
        description,
        epic,
    }
}
