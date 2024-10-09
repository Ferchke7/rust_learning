struct Counter{
    count: u32
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}
fn main() {
    let mut counter = Counter { count: 0 };
    for number in &mut counter {
        println!("Counter: {}", number);
    }
}