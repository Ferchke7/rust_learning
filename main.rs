trait Shape{
    fn area(&self) -> f64;
}
struct Circle{
    radius: f64
}
struct Rectangle{
    width: f64,
    height: f64
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14
    }
}
impl Shape for Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    //TODO look up tmrw
    let mut my_shape: &dyn Shape = &Circle { radius: 5.0 }; //decide what trait object save a link to the address
    println!("Circle, {}", my_shape.area());

    my_shape = &Rectangle { width: 10.0, height: 20.0 };
    println!("Rectangle {}", my_shape.area());

}