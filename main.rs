fn main() {
    let tom = Person { name: String::from("Tom"), age: 36 };
    tom.print_multiple_copies(3);
}
struct Person { name: String, age: u8 }
trait Printer{
    fn print_multiple_copies(&self, times: u8);
}
impl Printer for Person {
    fn print_multiple_copies(&self, mut number: u8){
        while number > 0 {
            println!("Person {}; age: {}",self.name, self.age);
            number-=1;
        }
    }
}