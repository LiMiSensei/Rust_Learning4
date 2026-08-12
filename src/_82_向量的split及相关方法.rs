fn retain_positive(x: &i32) -> bool {
    *x > 0
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut number = vec![-3, -2, -1, 0, 1, 2, 3];


    let vec= number.split_at(3);
    
    let vec = number.splitn(3,|e| e%2==0);

    let vec:Vec<_> = number.split(|x| *x % 2 == 0).collect();
    //反向拆分
    let vec:Vec<_> = number.rsplit(|x| *x % 2 == 0).collect();
    //拆分出
    let vec = number.split_off(3);

    for ss in vec{
        println!("{:?}",ss);
    }
}
