#[allow(dead_code)]
pub fn main() {
    let mut words = [
        "hello".to_string(),
        "world".to_string(),
        "how".to_string(),
        "are".to_string(),
        "you".to_string(),
    ];

    for i in &mut words{
        if i == "hello"{
            (*i).push_str(" world");
        }
        println!("{}",i);
    }


    println!("{:?}",words);
}
