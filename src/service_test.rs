use super::service::MySvc;

/*
#[tokio::test]
async fn test_service_uses_persistence() {
    use super::persistance::MockDBPersistence;
    
    let mut persistance = MockDBPersistence::default();
    
    persistance
        .expect_create_user()
        .times(1)
        .returning(|user| Ok(user));
   
    
    let p = Box::new(persistance);
    let service = MySvc::new(p);

    let result = service.create_user("uname", "email").await;

    let user = result.unwrap();
    assert_eq!(user.get_username(), "uname"); 
}
*/
