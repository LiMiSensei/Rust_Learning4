use std::panic;

#[allow(dead_code)]
pub fn main() {


    //故障安全模式
    let result = panic::catch_unwind(||{
        let buffer = [1,2,3,4,5,6];
        for i in 0..10{
            println!("{}",buffer[i]);
        }
    });

    println!("继续执行")
}
