fn main() {
    let mut tom = Person{
        name: "Tom".to_string(),
        age: 36
    };
    let bob = Person { name: "Bob".to_string(), age: 41 };
    let is_older = tom.is_older(&bob);
    if is_older{
        println!("STARSHE");
    }
    else {
        println!("ISMAIL PETUH");
    }
}
struct Person {
    name: String,
    age: u8
}
impl Person {
    fn is_older(&mut self, other: &Person) -> bool {
        self.age > other.age
    }
}