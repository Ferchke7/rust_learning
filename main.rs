fn main() {
    let tom = Person { name: String::from("Tom"), age: 36 };
    tom.print();
}
struct Person { name: String, age: u8 }
trait Printer{
    fn print(&self);
}
impl Printer for Person {
    fn print(&self) {
        println!("Person {}; age {}", self.name, self.age);
    }
}