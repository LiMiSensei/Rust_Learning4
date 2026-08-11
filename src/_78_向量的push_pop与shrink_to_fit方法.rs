#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut number = Vec::new();
    number.push(5);
    number.push(5);
    number.push(5);

    println!("{:?}", number);

    let val = number.pop();
    if let Some(val) = val {
        println!("{}", val)
    }
    //减少向量占用内存的方法
    number.shrink_to_fit();



}
