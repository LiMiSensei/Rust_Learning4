use std::rc::Rc;

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[allow(dead_code)]
pub fn main() {
   //引用计数智能指针RC<T>‘在以下场景中很有用：当一单线程应用程序的多要共享同一数据的所有权
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    let s1 = Rc::new(String::from("hello"));
    let s2 = Rc::clone(&s1);
    let s3 = Rc::clone(&s1);
    
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
}
