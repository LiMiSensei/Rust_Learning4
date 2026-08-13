fn find_biggesst_item<'a, 'b>(string: &'a [&'b str]) -> Option<&'b str> {
    let mut longest: Option<&'b str> = None;
    for item in string {
        if longest.is_none() || (item.len() > longest.unwrap().len()) {
            longest = Some(item);
        }
    }
    longest
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {



    let result;
    {
        let strings = ["1", "2", "3"];
        // &str 是静态生命周期
        //strings的生命周期没了，但是result使用的是result的生命周期，所有是有效的
        result = find_biggesst_item(&strings);
    }

    if let Some(v) = result {
        println!("AA");
    } else {
        println!("BBB")
    }
}
