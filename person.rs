struct Person {
    name: String,
    age: u8
}
impl Person {
    fn display($self) {
        println!("Name: {} Age {}", &self.name, &self.age)
    }
}