use std::num::IntErrorKind;
use std::str::FromStr;

fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    match i32::from_str(input) {
        Ok(n) => Ok(n),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => {Err("".to_string())}
            IntErrorKind::InvalidDigit => {Err("".to_string())}
            IntErrorKind::PosOverflow => {Err("".to_string())}
            IntErrorKind::NegOverflow => {Err("".to_string())}
            IntErrorKind::Zero => {Err("".to_string())}
            //IntErrorKind::NotAPowerOfTwo => {Err("".to_string())}
            _ => Err(format!("{}", e))
        },
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
     let result = parse_integer_from_string("9");

     match result {
         Ok(num) => println!(),
         Err(msg) => println!("Error: {}", msg),
     }
}
