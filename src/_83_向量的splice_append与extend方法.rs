
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut number = vec![-3, -2, -1, 0, 1, 2, 3];

    //拼接方法
    let vec= number.splice(..,vec![1,2,3]);
    
    let mut number = vec![-3, -2, -1, 0, 1, 2, 3];
    //追加
    let vec= number.append(&mut vec![1,2,3]);

    let vec= number.extend(4..6);
}
