fn main() {
    let numbers = [1,2,3,4,5,6,7];
    let slice = &numbers[1..5];
    println!("{:?}", slice);
}