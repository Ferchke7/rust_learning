truct Message { text: String}
struct Person { name: String}
 
trait Printer{  fn print(&self); }
  
impl Printer for Message{
    fn print(&self){
        println!("Message text: {}", self.text);
    }
}
 
impl Printer for Person{
    fn print(&self){
        println!("Person name: {}", self.name);
    }
}
 
trait ConsolePrinter{  fn console_print(&self); }
impl<T:Printer> ConsolePrinter for T{
    fn console_print(&self){
        println!("******Печать на консоль*****");
        self.print();
        println!(); // для отделения строк при выводе на консоль
    }
}
 
fn main(){
      
    let mes = Message {text: String::from("Hello METANIT.COM")};
    mes.console_print();
 
    let tom = Person{name: String::from("Tom")};
    tom.console_print();
}