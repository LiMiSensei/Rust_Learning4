#[derive(Debug,Default)]
struct Person{
    name :String,
    age:u8,
    is_male:bool,
    heighr:f32,
    initial:char,

}
#[allow(dead_code)]
pub fn main() {
    let user = Person::default();
    let p1 = Person{
        name: "".to_string(),
        age: 0,
        ..Default::default()
    };
    println!("{:?}",user);

}
