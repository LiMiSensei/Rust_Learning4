#![feature(extract_if)]
fn filter(e:&mut i32) -> bool{
    *e > 5
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut v = vec![1,2,3,4];
    for e in v.drain(..){
        println!("{}",e)

    }

    let mut  number = vec![1,2,3,4,5,6];
    let c1 :Vec<_> = number.extract_if(..,filter).collect();
    println!("{:?}",number);

}
