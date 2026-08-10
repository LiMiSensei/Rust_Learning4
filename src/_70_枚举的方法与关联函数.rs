struct Rectangle {
    h: f32,
    w: f32,
}
enum Shape {
    Circle { x: f32, y: f32, radius: f32 },
    Rectangle(Rectangle),
    Square(f32, f32, f32),
}

impl Shape {
    fn new_circle(x: f32, y: f32) -> Shape {
        Shape::Circle { x, y, radius: 0.0 }
    }

    fn area(self: &Shape) -> f32 {
        match self {
            Shape::Circle { x, y, radius } => std::f32::consts::PI * radius * (x * y),
            Shape::Rectangle(rect) => rect.w * rect.w + rect.h * rect.h,

            Shape::Square(x, y, radius) => x * y * radius,
        }
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let new_share = Shape::new_circle(0.32, 0.12);
}
