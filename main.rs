struct Person<T>{
  
    id: T,
    name: String
}
impl<T> Person<T>{
       
    fn clone_with_name<V>(&self, person_id: V) -> Person<V>{
        Person{ id: person_id, name: self.name.clone()}
    }
}
fn main(){
      
    let tom = Person{id:1, name: String::from("Tom")};
    let tom2 = tom.clone_with_name("235qwerty");
    println!("Tom2 Info. Id: {}  Name: {}", tom2.id, tom2.name); // Tom2 Info. Id: 235qwerty  Name: Tom
}