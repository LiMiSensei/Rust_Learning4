#[allow(dead_code)]
pub fn main() {

    //============ 反转数组 ================
    //数据
    let my_array= [1,2,3,4,5,6,7,8];

    //可变数组
    let mut clone_array = my_array;
    println!("{:?}", my_array);

    //获取可变切片引用
    let reverse = &mut clone_array;
    reverse.reverse();//反转数组

    //打印
    println!("{:?}", reverse);

    //============ 连接两个数组 ================
    let my_array1= [1,2,3,4,5,6,7,8];
    let my_array2= [1,3,5,7,9,2,4,6];
    let concat_array = [my_array1,my_array2].concat();
    println!("{:?}", concat_array);
    


}
