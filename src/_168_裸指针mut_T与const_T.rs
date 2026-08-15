
#[allow(dead_code)]
pub fn main() {
    
    let mut x = 10;
    let ptr = &mut x as *mut i32; // ✅ 改为 *mut i32

    unsafe {
        *ptr = 30;
        println!("{}", x); // 输出 30
    }

    
}
