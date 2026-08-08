#[allow(dead_code)]
fn store_data_heap(data:u32){
    let heap_value = Box::new(data);
    println!("{}", heap_value);
    //内存释放
}
#[allow(dead_code)]
pub fn main() {
    let x: Option<&i32> = None;
    let original_ptr = Box::new(42);
    let copied_ptr = &original_ptr;

    //drop(original_ptr);

    println!("{:?}", *copied_ptr);
}
