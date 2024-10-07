struct Circle {
    radius: f64,
}
 
trait Shape {
    fn area(&self) -> f64;
    fn default_shape() -> Self; // определяем ассоциированную функцию
}
 
 
impl Shape for Circle {
    fn area(&self) -> f64 { self.radius * self.radius * 3.14}
 
    fn default_shape() -> Self { // реализуем ассоциированную функцию
        Circle { radius: 1.0 }
    }
}
 
 
fn main() {
 
    // создаем объект Circle и вычисляем его площадь
    let circle = Circle { radius: 2.0 };
    let area = circle.area();
    println!("Circle area: {}", area);  // Circle area: 12.56
 
 
    // создаем объект Circle по умолчанию и вычисляем его площадь
    let default_circle = Circle::default_shape();
    let default_area = default_circle.area();
    println!("Default Circle area: {}", default_area);  // Default Circle area: 3.14
 
}