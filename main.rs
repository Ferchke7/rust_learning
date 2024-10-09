use std::ops::Add;

struct Counter{
    value: u32
}
impl Add for Counter{
    type Output = Counter; // Generating type of value
    fn add(self, other: Counter) -> Counter{
        Counter {
            value: self.value + other.value
        }
    }
}

fn main() {
    let counter1 = Counter{value: 5};
    let counter2 = Counter{value: 15};
    let counter3 = counter1 + counter2;
    println!("{}", counter3.value);
}