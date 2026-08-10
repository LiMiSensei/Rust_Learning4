

//对于有可能得不到的值  使用Option
fn find_biggest_item<'a>(string:&'a[&'a str]) ->Option<&'a str>{
    let mut longest:Option<&'a str> = None;
    for item in string{
        if longest.is_none() || (item.len() > longest.unwrap().len()){
            longest = Some(item);
        }
    }
    longest
}

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {

    let string = ["Mango","Banana","Apple"];
    let biggest_item = find_biggest_item(&string);
    println!("{:?}",biggest_item);
}
