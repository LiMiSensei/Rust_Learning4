#[allow(dead_code)]
pub fn main() {
    let s1 = String::from("hello");

    let ch = s1.chars().next().unwrap();
    println!("{}", ch);
    let ch = s1.chars().nth(1); //获取指定索引
    println!("{:?}", ch);

    if let Some(c) = ch {
        println!("{:?}", c)
    }else{
        println!("no ")
    }

    let messages = String::from("hello world");
    let byt_slice = messages.as_bytes();
    for byt in byt_slice{
        println!("{}",byt);//印第安字符比较占字节
    }
}
