use std::any::type_name;
fn type_of<T>(_:&T)->String{
    format!("{}", type_name::<T>())
}
#[allow(dead_code)]
pub fn main() {
    let i8:i8 = 12; //-127 to 127
    let u8:u8 = 12;

    let i16:i16 = 12; //-32768 to 32767
    let u16:u16 = 12;

    let i32:i32 = 12; //-2147483648 to 2147483647
    let u32:u32 = 12;

    let i64:i64 = 12;//-9223372036854775808 t09223372036854775807
    let u64:u64 = 12;

    let i128:i128 = 12;
    let u128:u128 = 12;

    let isize:isize = 12;//平台相关
    let usize:usize = 12;

    let f32:f32 = 12.0;//-3.4*10^38 to 3.4*10^38
    let f64:f64 = 12.0;//-1.8*10^308 to 1.8*10^308

    let num1:i32 = 12;
    let num2:i32 = 12;
    let sum:u8 = (num1 + num2) as u8;
    println!("数据类型的转换========");
    println!("num1:num2:{}", num1);

    println!("获取类型========");
    println!("{}",type_of(&num1));
    println!("{}",type_of(&sum));

    println!("可变变量========");
    let mut num = 10;
    num = 20;

    println!("整数和浮点相乘========");
    let num1 = 20;
    let num2 = 2.5;
    let num3 :&i32 = &num1;
    let mul = num2 * num1 as f64;//没有隐式转换
    let mul = num3 * num1;
    println!("{}",mul);
   
}

