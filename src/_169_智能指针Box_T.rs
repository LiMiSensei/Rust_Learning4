
#[allow(dead_code)]
pub fn main() {
    let val = 100;

    let mut p = Box::new(val);

    println!("{}",p);
    *p = 200;
    println!("{}",p);
    println!("{}",p);



}
