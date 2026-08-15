#[allow(dead_code)]
pub fn main() {
    let s = "Good".to_string();
    let box_s = Box::new(s);//s 的所有权已被转移
    println!("{}", box_s)


    
}
