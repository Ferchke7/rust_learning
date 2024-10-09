struct Circle {
    radius: f64,
}
trait Drawable<T> {
    fn draw(&self, figure:&T);
}
struct CircleGui;
impl Drawable<Circle> for CircleGui{
    fn draw(&self, figure: &Circle){
        println!("Draw a circle {}", figure.radius);
    }
}
fn main() {
    let gui = CircleGui{};
    let circle = Circle { radius: 2.0 };
    gui.draw(&circle);
}