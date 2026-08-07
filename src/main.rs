#[path = "_1_简介.rs"]
mod _1_简介;
#[path = "_2_Rust的内存安全意味着什么第1部分.rs"]
mod _2_Rust的内存安全意味着什么第1部分;

#[path = "_4_创建_构建和运行rust程序.rs"]
mod _4_创建_构建和运行rust程序;

#[path = "_5_Rust中的打印相关宏.rs"]
mod _5_Rust中的打印相关宏;

#[path = "_6_format格式化和命名占位符.rs"]
mod _6_format格式化和命名占位符;

fn main() {
    if false {
        _1_简介::main();
        _2_Rust的内存安全意味着什么第1部分::main();
        _4_创建_构建和运行rust程序::main();
        _5_Rust中的打印相关宏::main();
    }

   
    _6_format格式化和命名占位符::main();
}
