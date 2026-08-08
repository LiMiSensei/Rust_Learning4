#[allow(dead_code)]
pub fn main() {
    let mut mun1 = 50;
    let ref_of_num1 = &mut mun1;
    *ref_of_num1 = 100;

    print!("{}", mun1);

    println!("不允许多个可变借用");

    let mut num = 50;
    let ref1 = &num;
    let ref2 = &num;
    let ref3 = &num;

    println!("{},{},{}", ref1, ref2, ref3);
}
