#[allow(dead_code)]
pub fn main() {
    //drop方法释放内存
    let array1 = [1,2,3,4,5];

    let array2 = [String::from("hello"), String::from("world")];

    let item = array1[1];
    println!("{}",item);

    for str in &array2 {
        println!("{}",str)
    }

    println!("{:?}",array2);
    
    
    let msg = "Hello".to_string();
    let ref1 = &msg;
    let ref2 = ref1;
    
    if ref1 ==ref2{
        println!("ref1 == ref2")
    }
}
