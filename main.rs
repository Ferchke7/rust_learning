fn main() {
    let username = String::from("Sam");
    {
        let default_name = String::from("Tom");
        let checked_username = check_name(&username, &default_name);
        println!("username {}", checked_username);
    }
}
fn check_name<'a>(name: &'a str, default: &'a str) -> &'a str {
    if name == "admin" {default}
    else {name}
}