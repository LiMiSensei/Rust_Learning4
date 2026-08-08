#[allow(dead_code)]
pub fn main() {

    let age = 19;
    if age > 18{
        println!("Age is greater than 18");
    }else {
        println!("Age is larger than 18");
    }

    //类似三元运算符，但是可以多语句
    let messages = if age < 18{"666"}else{"8888"};


}
