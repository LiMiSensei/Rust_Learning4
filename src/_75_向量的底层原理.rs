#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut v: Vec<i32> = vec![];

    let ca = v.capacity();
    let len = v.len();
    let ptr = v.as_ptr();
}
