struct Person<'a> {
    name: &'a str,
}

fn main() {
    let tom = Person { name: "Tom" };

    println!("{}", tom.name);
}