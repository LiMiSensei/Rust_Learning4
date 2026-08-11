fn retain_positive(x: &i32) -> bool {
    *x > 0
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut number = vec![-3, -2, -1, 0, 1, 2, 3];
    let vec:Vec<_> = number.split(|x| *x % 2 == 0).collect();

    for ss in vec{
        println!("{:?}",ss);
    }
}
