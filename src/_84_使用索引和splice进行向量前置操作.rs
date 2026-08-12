#[derive(PartialEq,PartialOrd)]
struct Person{
    name:String,
    age:u32
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    if vec1 < vec2 {
        println!("vec1 < vec2");
    }else {
        println!("vec1 < vec2");
    }


    let person1 = Person{name:"John".to_string(),age:33};
    let person2 = Person{name:"Bob".to_string(),age:34};
    let vec1 = vec![person1];
    let vec2 = vec![person2];

    println!("{}",vec1<vec2);
}
