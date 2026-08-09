#[allow(dead_code)]
pub fn main() {
    let my_String = String::new();
    // 0x200 5 capactyu   //内存地址 字符长度 字面量
    let mut num = 3.1415.to_string();

    let s1 = String::from("hello");
    let s2 = s1;//转移所有权
    println!("s1失效，s2能用{}",s2);

    let s1 = 10;
    let s2 = s1;
    println!("这里是复制，不是所有权转移{},{}",s1,s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("克隆，s1和s2都能用{}，{}",s1,s2);





}
