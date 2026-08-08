#[allow(dead_code)]
pub fn main() {
    let name = "world";
    let age = 18;

    //format！
    let meg = format!("{}", name);
    
    println!("My name is {} and I am {}", meg, age);
    
    //占位符 使用反斜杠行进行符
    println!(
        "My name is {user_name} \
        and I am {user_age}",
        user_name = meg,
        user_age = age
    );
}
