struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

fn fun<'a,'b,'c>(data:&'a MyStruct<'b,'c>) -> &'a str{
    data.data1
}

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let data1 = "Hello";
    let data2 = "World";
    let my_struct = MyStruct { data1, data2 };

    let ret = fun(&my_struct);

    println!("{}", my_struct.data1);
}
