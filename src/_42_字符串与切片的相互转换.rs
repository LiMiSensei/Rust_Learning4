fn print_string(s:&str){
    println!("{}",s);
}
#[allow(dead_code)]
pub fn main() {
    //字符窜转切片

    let mut s = String::from("hello world");
    let slice = &s;

    print_string(&slice);

    let slice2 = s.as_str();
    let slice3 = s.as_mut_str();

    let s = "Good Morning";
    let string = String::from(s);

    
}
