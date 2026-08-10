#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main(){
    let person = Person{ name: "".to_string(), age: 0 };


    match person {
        Person{age:30,name:_} => println!(""),
        Person{ref name,age:35} => println!("{},{}", name, 35),
        _ => println!("")
    }
}


