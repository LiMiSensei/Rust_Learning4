#[allow(dead_code)]
pub fn main() {
    //! & | << >> ^
    // !：非运算符 用于布尔
    // & 保0运算符号
    // | 保1运算符号

    let a = 0; //0000
    let b = 1; //0001

    let c = a<<b;

    let a = 0x80_u8;//1000 0000
    let b = a>>2;   //0010 0000

    let num = 0x00ABCDF;
    let mask = 0x1FF << 4;

    let res = ((num & mask) >> 4) & 0x1FF;
    println!("{:#x}",res);


}
