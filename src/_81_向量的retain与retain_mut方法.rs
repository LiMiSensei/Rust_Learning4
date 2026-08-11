fn retain_positive(x: &i32) -> bool{
    *x > 0
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut number = vec![-3,-2,-1,0,1,2,3];
    number.retain(retain_positive);
    number.retain(|x| *x > 0);
    println!("{:?}",number);

    
}
