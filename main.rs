use std::collections::HashMap;

fn main() {
    let raw_data = vec![ ("Alice", 35), ("Tom", 39) ];
    let people: HashMap<_,_> = raw_data.iter().cloned().collect();

    println!("{:?}",people);
}