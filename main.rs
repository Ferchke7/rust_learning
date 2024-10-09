struct Person<T>{
 
    id: T,
    name: String
}
impl Person<u32>{
    fn compare_id(&self, user_id: u32) -> bool{
        self.id == user_id
    }
}
fn main(){
     
    let tom = Person{id:1, name: String::from("Tom")};
    let result1 = tom.compare_id(1);
    println!("result1: {}", result1);   // result1: true
     
    let bob = Person{id:4, name: String::from("Bob")};
    let result2 = bob.compare_id(1);
    println!("result2: {}", result2);   // result2: false
}