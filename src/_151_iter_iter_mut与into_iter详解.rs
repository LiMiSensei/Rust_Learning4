#[allow(dead_code)]
pub fn main() {
    let msg = "Good Morning".to_string();

    let vec_of_chars: Vec<_> = msg.chars().collect();

    println!("{:?}", vec_of_chars);
}
