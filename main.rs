fn main() {
    let tom = Person{
        name: "Tom".to_string(),
        age: 36
    };
    tom.display();
    tom.display();
    tom.say_hello(&tom, 19);
    let bob = Person { name: "Bob".to_string(), age: 41 };
    bob.display();
}
struct Person {
    name: String,
    age: u8
}
impl Person {
    fn say_hello(&self, other: &Person, hour: u8) {
        if hour < 16{
            println!("Hi, {}!", other.name);
        }
        else {
            println!("Good evening, {}", other.name);
        }
    }

    fn display(&self){
        println!("Name: {} Age {}", &self.name, &self.age)
    }
}