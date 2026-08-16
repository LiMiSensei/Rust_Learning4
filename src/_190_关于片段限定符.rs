macro_rules! hello {
    () => {{}
        println!("Hello World");
    };
    ($name:literal)=>{
        println!("{}",$name);
    };

    ($name:literal,$name2:literal)=>{
        println!("{}，{}",$name,$name2);
    };

    ($($name:literal),+) =>{
        $(
            println!("Hello,{}",$name);
        )+
    };


    ($name1:expr,$name2:expr,$($name:expr), + $(,)*) => {
        $(
            println!("Hello,{}",$name);
        )+
    };
}
#[allow(dead_code)]
pub fn main() {
    //宏是允许你编写高效Rust代码的强大工具。
    //宏是编程特性或编程构造，用于支持它们的语言中的元编程。
    hello!("hello");
    hello!("hello","6666");
    hello!("hello","6666","sadas");

    hello!((1+2),(3+4),(99+1));
}

