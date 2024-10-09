fn main() {
    let results1 = receive(3);
    println!("{}", results1);

    let results2 = receive("somet");
    println!("{}", results2);
}
fn receive<T>(item: T) -> T{
    item
}