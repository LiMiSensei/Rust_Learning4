#[allow(dead_code)]
pub fn main() {
    let array1: [i32; 4] = [1, 2, 3, 4];
    let slice = &array1[..];//切片引用
    println!("{:?}", slice); //动态切片大小

    let array1: [i32; 4] = [1, 2, 3, 4];
    //let slice = array1[..];//这样是不行的因为没有已知大小
}
