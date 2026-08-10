#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}
#[allow(dead_code)]
pub fn main() {
    let name = String::from("Jack");
    let age = 32;
    let add = String::from("123");

    let mut person = Person { name, age, address:add };

    person.name = String::from("Jack");

    let _name = person.name.clone();//Move了

    println!("{}", person.age);
    println!("{}", person.name);
    println!("{}", person.address);
    println!("{:?}", person);
}
