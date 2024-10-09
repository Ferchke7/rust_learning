struct Person<T> {
    id: T,
    name: String
}

fn main() {
    let tom = Person { id: 245, name: "Tom".to_string() };
    println!("id: {} name {}", tom.id, tom.name);

}