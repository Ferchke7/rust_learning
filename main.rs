//Conditional conformance
trait Convertible<T> {
    fn convert(&self) -> T;
}
struct Celsius(f64);
struct Fahrenheit(f64);

impl Convertible<Fahrenheit> for Celcius{
    fn convert(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 1.8 + 32.0)
    }
}

impl Convertible<Celcius> for Fahrenheit{
    fn convert(&self) -> Celcius {
        Celcius((self.0 - 32.0) / 1.8)
    }
}
fn main() {
    let celsius = Celcius(100.0);

    let converted_fahrenheit: Fahrenheit = celsius.convert();
    println!("100 in fahrenheit is {:.2}", converted_fahrenheit.0)
}