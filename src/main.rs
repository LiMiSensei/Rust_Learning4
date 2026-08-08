#[path = "_1_简介.rs"]
mod _1_简介;
#[path = "_2_rust的内存安全意味着什么第1部分.rs"]
mod _2_rust的内存安全意味着什么第1部分;

#[path = "_4_创建_构建和运行rust程序.rs"]
mod _4_创建_构建和运行rust程序;

#[path = "_5_rust中的打印相关宏.rs"]
mod _5_rust中的打印相关宏;

#[path = "_12_字符串的r和r标记.rs"]
mod _12_字符串的r和r标记;
#[path = "_6_format格式化和命名占位符.rs"]
mod _6_format格式化和命名占位符;
#[path = "_7_练习1.rs"]
mod _7_练习1;
#[path = "_8_实用的cargo工具.rs"]
mod _8_实用的cargo工具;
#[path = "_9_十六进制和二进制格式打印.rs"]
mod _9_十六进制和二进制格式打印;

#[path = "_13_变量可变性与数据类型详解.rs"]
mod _13_变量可变性与数据类型详解;
#[path = "_14_as关键字与ascii值存储.rs"]
mod _14_as关键字与ascii值存储;
#[path = "_15_字符数据类型.rs"]
mod _15_字符数据类型;
#[path = "_17_数组与数组遍历.rs"]
mod _17_数组与数组遍历;

#[allow(dead_code)]
fn main() {
    if false {
        _1_简介::main();
        _2_rust的内存安全意味着什么第1部分::main();
        _4_创建_构建和运行rust程序::main();
        _5_rust中的打印相关宏::main();
        _6_format格式化和命名占位符::main();
        _7_练习1::main();
        _8_实用的cargo工具::main();
        _9_十六进制和二进制格式打印::main();
        _12_字符串的r和r标记::main();
        _13_变量可变性与数据类型详解::main();
        _14_as关键字与ascii值存储::main();
        _15_字符数据类型::main();
    }


    _17_数组与数组遍历::main();
}
