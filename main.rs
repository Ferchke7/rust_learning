#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}
fn main() {
    let tom = Person{name:"Tom".to_string(), age: 40};
    println!("{:?}", tom);
}