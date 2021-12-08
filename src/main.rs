use rustserver::models::user;

fn main() {
    let u = user::User::new("name", "address", "30");
    println!("Hello, world! {}", u);
    println!("User: {}", u);
}
