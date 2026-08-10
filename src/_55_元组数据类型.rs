#[allow(dead_code)]
pub fn main() {
    let my_tuple = (1, "s", false);
    println!("{:?}", my_tuple);

    let number = my_tuple.0;
    let messages = my_tuple.1;
    let is_valid = my_tuple.2;

    //元组解构
    let (number, messages, is_valid) = my_tuple;

    let mut number = (1, 2, 3);
    increment_number(&mut number); //使用可变引用
    println!("{:?}", number);

    increment_number1(&mut number);
    println!("{:?}", number);
}

fn increment_number(num: &mut (i32, i32, i32)) {
    num.0 += 1;
    num.1 += 1;
    num.2 += 1;
}

fn increment_number1(num: &mut (i32, i32, i32)) {
    let (mut a, mut b, mut c) = *num;
    a += 1;
    b += 1;
    c += 1;
    *num = (a, b, c);
}
