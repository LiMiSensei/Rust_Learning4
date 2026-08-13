fn add_string(s1:&str,s2:&str) -> Option<String> {
    if s1.is_empty() || s2.is_empty(){
        None
    }else {
        Some(format!("{}{}", s1,s2))
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main(){
    let res = add_string("hello world", "world").ok_or("");

    
}