fn main() {
    let tom = Person { name: String::from("Tom"), age: 36 };
    let tom_preview = tom.preview();
    println!("{}", tom_preview);
}
struct Person { name: String, age: u8 }
trait Printer{
    fn preview(&self) -> String;
}
impl Printer for Person {
    
    fn preview(&self) -> String{
        format!("[Предпросмотр] Person {}; age: {}", self.name, self.age)
    }
}