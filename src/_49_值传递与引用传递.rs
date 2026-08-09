#[allow(dead_code)]
pub fn main() {
    let x = 42;
    let ref1 = &x;
    let ref2 = &ref1;
    let ref3 = &ref2;

    println!("{},{},{}",ref1,ref2,ref3);//都等于42

    

    if ref1 == (*ref2){//解引用
        println!("ref1 == (*ref2)")
    }
}
