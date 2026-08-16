
#[allow(dead_code)]
pub fn main() {
    let boxed_int = Box::new(100);
    let raw_ptr = Box::into_raw(boxed_int) as *const i32;

    unsafe {
        let _ = Box::from_raw(raw_ptr as *mut i32);
    }
    
    
    let vec_of_i32 = vec![10;100];
    
    let boxed_i32 = vec_of_i32.into_boxed_slice();
    println!("{:?}", boxed_i32);

}
