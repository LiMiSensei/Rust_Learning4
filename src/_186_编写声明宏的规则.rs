macro_rules! hello {
    () => {{}
        println!("Hello World");
    };
    ($name:literal)=>{
        println!("{}",$name);
    };
}
#[allow(dead_code)]
pub fn main() {
    //宏是允许你编写高效Rust代码的强大工具。
    //宏是编程特性或编程构造，用于支持它们的语言中的元编程。
    hello!("hello");
}

