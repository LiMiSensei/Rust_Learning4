#[allow(dead_code)]
pub fn main() {
    let point = (0, 1);
    if let (0, 0) = point {
        println!("1")
    } else {
        println!("2")
    }

    if let (_, y @ 1..4) = point {
        println!("3")
    } else {
        println!("4")
    }
}
