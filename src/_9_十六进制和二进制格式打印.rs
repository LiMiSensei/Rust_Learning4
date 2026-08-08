

#[allow(dead_code)]
pub fn main() {
    //省略精度
    let real_valuew = 3.1415926;
    let out1 = format!("{:.2}",real_valuew);
    let out2 =format!("{:.6}",real_valuew);
    let out3 = format!("{}",real_valuew);
    println!("省略精度:{},{},{}", out1, out2, out3);
    // 进制转换
    let real_valuew = 1235435;
    let out1 = format!("{:#X}",real_valuew);
    let out2 =format!("{:#x}",real_valuew);
    let out3 = format!("{:x}",real_valuew);
    println!("进制转换:{},{},{}", out1, out2, out3);

    // 二进制
    let out1 = format!("{:b}",real_valuew);//二进制
    println!("{}", out1);
}

