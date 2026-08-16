#[allow(dead_code)]
pub fn main() {
    //如果你只是想临时使用裸指针，而又不想放弃BOX的所有权，那么可以考虑借用BOx智能指针
    let mut boxed_int = Box::new(5);
    let immutable_raw_ptr = &(*boxed_int) as *const i32;

    unsafe {
        println!("immutable_raw_ptr: {:?}", *immutable_raw_ptr);
    }
    let mutable_raw_ptr = &mut (*boxed_int) as *mut i32;

    unsafe {
        *mutable_raw_ptr = 30;
        println!("immutable_raw_ptr: {:?}", *mutable_raw_ptr);
    }

    unsafe {
        println!("immutable_raw_ptr: {:?}", *immutable_raw_ptr);
    }
}
