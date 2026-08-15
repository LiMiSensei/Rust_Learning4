struct MyStruct{
    val:i32,
    action:Box<dyn Fn(i32) -> i32>, //允许动态分发
}
#[allow(dead_code)]
pub fn main() {


    let y = 10;
    let closure = Box::new (move|x|x + y);

    let s = MyStruct{
        val: 10,
        action: closure,
    };
}
