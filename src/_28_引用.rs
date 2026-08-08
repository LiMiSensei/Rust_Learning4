
#[allow(dead_code)]
pub fn main() {
    let value = 42;//value : 42  地址：0x200
    let ref_of_value = &value;//:0x200
    let ref_of_value = &ref_of_value;//:0x200
    let ref_of_value = &ref_of_value;//:0x200
    println!("地址：{:p}", ref_of_value);
    let mut value1 = ***ref_of_value;
    value1 = 100;
    println!("{}", value1);
    println!("{}", value);

}

