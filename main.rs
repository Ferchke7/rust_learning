fn main() {
    let tom = Person::create("Tom", 36);
    println!("Name {} Age {}", tom.name, tom.age);
    
}
struct Person { name: String, age: u8 }
impl Person{
    fn create(user_name: &str, user_age: u8) -> Person{
        Person{
            name: String::from(user_name),
            age: user_age
        }
    }
}