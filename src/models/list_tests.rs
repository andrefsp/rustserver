use super::list::List;
use super::user::User;
use super::items;


#[test]
fn test_can_create_list() {
    let user = User::new("andrefsp", "email@email.com", 32);
    let list = List::new(1, "todo", user.get_id());

    assert_eq!(list.get_id(), 1);
    assert_eq!(list.get_name(), "todo");
    assert_eq!(list.get_user_id(), 32);
}


#[test]
fn test_can_add_items() {   
    let user = User::new("andrefsp", "email@email.com", 32);
    let mut list = List::new(1, "todo", user.get_id());
    
    let meeting = items::new_meeting(
        12, String::from("subject")
    );
    let task = items::new_task(
        12, String::from("description"), String::from("epic")
    );

    list.add_item(meeting);
    list.add_item(task);
}
