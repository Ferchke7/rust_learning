fn main() {
    let mut tom = Person{
        name: "Tom".to_string(),
        age: 36
    };
    println!("Do izmenenie {}", tom.age);
    tom.change_age(22);
    println!("Posle izmenenie {}", tom.age)
}
struct Person {
    name: String,
    age: u8
}
impl Person {
    fn change_age(&mut self, age: u8) {
        self.age = age;
    }
}