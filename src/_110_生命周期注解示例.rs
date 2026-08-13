fn extend_string<'a>(original:&'a mut String,data:&'a str) -> &'a str{
    data
}
fn foo(x:&i32)-> &i32{
    let y = 50;
    &50//这是一个常量生命周期
    //不能返回局部引用
}

fn test<'a>(x: &'a mut i32) -> &'a i32{
    x
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    //表示输入引用和返回函值之间的关系

    let mut a  = &4;
    let mut b = 10;
    {
        a = test(&mut b);
    }

    println!("{}",a);

}
