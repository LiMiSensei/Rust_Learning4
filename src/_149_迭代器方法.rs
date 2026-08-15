#[allow(dead_code)]
pub fn main() {
    let mut number = [1, 2, 3, 4, 5];

    for i in 0..number.len() {
        number[i] *= number[i]
    }

    println!("{:?}", number);
}
