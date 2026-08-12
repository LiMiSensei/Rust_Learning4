fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    if s1.is_empty() || s2.is_empty() {
        return Err("Err".to_string());
    }

    let c = format!("{},{}", s1, s2);
    Ok(c)
}

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    // enum Result<T,E>{
    //     OK(T),
    //     Err(E)
    // }

    let c = add_strings("s", "s2");

    match c {
        Ok(v) => println!(),
        Err(e) => println!("{}", e),
    }
}
