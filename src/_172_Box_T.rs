#[allow(dead_code)]
pub fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let s1: Box<[i32]> = Box::from(&array[1..3]);
    //等同以上
    let s1: Box<[i32]> = array[1..3].to_vec().into_boxed_slice();
    
    let s2 = &s1[..];
    
    println!("s2: {:?}", s1);
    println!("s2: {:?}", s2);
}
