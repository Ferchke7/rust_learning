struct Circle { radius: f64 }
struct Rectangle { width: u32, height: u32 }

trait Shape {
    type Unit;
    fn area(&self)->Self::Unit;
}
impl Shape for Circle{
    type Unit = f64;
    fn area(&self) -> Self::Unit { self.radius * self.radius * 3.14 }
}
impl Shape for Rectangle {
    type Unit = u32;
    fn area(&self) -> Self::Unit { self.width * self.height }
}
fn main() {
    let circle = Circle{radius: 10.0};
    println!("Area = {} ", circle.area());
    let rect = Rectangle{width:10, height:20};
    println!("Area = {}", rect.area());

}
//Alternative way to assocative type
struct Circle;
struct Rectangle;
trait Shape {}
impl Shape for Circle{ }
impl Shape for Rectangle{ }
 
struct CircleGeometry;
struct RectangleGeometry;
 
trait Geometry{
    type ShapeType: Shape;
    fn create_shape()-> Self::ShapeType;
}
 
impl Geometry for CircleGeometry{
    type ShapeType = Circle;
    fn create_shape() -> Self::ShapeType { Circle}
}
impl Geometry for RectangleGeometry{
    type ShapeType = Rectangle;
    fn create_shape() -> Self::ShapeType { Rectangle}
}