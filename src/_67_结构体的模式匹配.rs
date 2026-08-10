struct Point {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
pub fn main() {
    let p = Point { x: 0, y: 0 };

    match p {
        Point { x, y }if y > 0 => println!(),
        Point { x:x @ 0,.. } => println!("x is {}", x),
        _ => println!("_")
    }
}


