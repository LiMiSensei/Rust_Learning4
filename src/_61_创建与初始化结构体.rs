struct Person {
    name: String,
    age: u32,
    address: String,
}
#[allow(dead_code)]
pub fn main() {
    let user = Person {
        name: "TTT".to_string(),
        age: 32,
        address: "val".to_string(),
    };
}
