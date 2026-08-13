trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    readus: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        2.0 * self.readus * std::f64::consts::PI
    }
}

fn print_area(shape: &dyn Shape) {
    println!("area: {}", shape.area());
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let circle = Circle { readus: 2.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    
    let vec_shaper : Vec<&dyn Shape> = vec![&circle, &rectangle];
    println!("area is {}", rectangle.area());
}
