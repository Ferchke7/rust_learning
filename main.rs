struct Person<T>{
    id: T,
    name: String
}
impl<T> Person<T>{
    fn get_id(&self) -> &T{
        &self.id
    }
}
fn main() {
    let user = Person {id: "be something", name: String::from("1") };
    
    let anotherUser = Person {id: 1, name: "Tom".to_string() };
    println!("{}",user.get_id());
    println!("{}",anotherUser.get_id());
    
}