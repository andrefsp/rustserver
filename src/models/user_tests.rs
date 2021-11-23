#[test]
pub fn test_user_init() {
    use super::user::User;
    
    let u = User::new("andrefsp", "email@email.com", 32);
    assert_eq!(u.get_id(), 32);
    assert_eq!(u.get_email(), "email@email.com");
    assert_eq!(u.get_username(), "andrefsp")
}

#[test]
pub fn test_user_list() {
    use super::user::User;

    let mut v = vec![
        User::new("andrefsp1", "email1@email.com", 31),
        User::new("andrefsp3", "email3@email.com", 33),
        User::new("andrefsp2", "email2@email.com", 32),
    ];

    v.sort();

    assert_eq!(v[0].get_id(), 31);
    assert_eq!(v[1].get_id(), 32);
    assert_eq!(v[2].get_id(), 33);
}
