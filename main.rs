trait Printer { fn print(&self); }
trait Sender { fn send(&self); }

struct Message { text: String }
impl Printer for Message{
    fn print(&self){
        println!("message: {}", self.text);
    }
}
impl Sender for Message{
    fn send(&self){
        println!("sent ^^");
    }
}

fn process(obj: &(impl Printer + Sender)) {
    obj.print();
    obj.send();
}
fn main() {
    let mes = Message { text: String::from("Hello")};
    process(&mes);
}