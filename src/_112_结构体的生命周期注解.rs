struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let data1 = "Hello";
    let data2 = "World";
    let my_struct = MyStruct { data1, data2 };

    println!("{}", my_struct.data1);
}
