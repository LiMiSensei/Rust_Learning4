#[allow(dead_code)]
pub fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    let s2 = String::from("Morning");
    let s3 = s1.clone() + &s2; //拼接时必须借用

    let s4 = s3.clone() + "jia";//string + &str //s3会失去所有权

    
    let s5 = format!("{}-{}-{}", s1, s2, s3);//
    println!("{}", s3);
}
