use std::collections::HashMap;

fn main() {
    let people = HashMap::from([
        ("Alice",35),
        ("Tom", 39)
    ]);
    println!("{:?}", people);
}