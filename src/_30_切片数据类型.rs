#[allow(dead_code)]
pub fn main() {
    let my_array: [i32; 4] = [1, 2, 3, 4];

    // 可以 多个不可变借用
    let s1 = &my_array[1..=3];//不可变切边借用
    let s2 = &my_array[..];//不可变切边借用
    let s3 = &my_array[0..1];//不可变切边借用

    println!("{:?}",s1);//切片
    println!("{}",s1[0]);//读取

    //切片是引用某个对象
    let mut sum = 0;
    for  &i in s1{  //切片进行 引用
        sum += i; //如果没有 &i i会隐式解引用
    }
    println!("sum:{}",sum)
}
