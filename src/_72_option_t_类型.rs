

//对于有可能得不到的值  使用Option
fn find_value(array:&[i32],taarget:i32) -> Option<i32>{
    for (index,value) in array.iter().enumerate() {
        if *value == taarget{
            return Some(index as i32)
        }
    }
    None
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let v1 = Option::Some(20);
    let v2 :Option<String> = Some("K".to_string());
    let v3:Option<i32> = None;
    let v4 = Option::<i32>::None;
}
