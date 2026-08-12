use std::ffi::NulError;
use std::io;
use std::io::Write;
use std::num::IntErrorKind;
use std::str::FromStr;


fn convert_err(e:std::num::ParseIntError) -> io::Error{
    io::Error::new(io::ErrorKind::InvalidData, e)
}
fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    match i32::from_str(input) {
        Ok(n) => Ok(n),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err("".to_string()),
            IntErrorKind::InvalidDigit => Err("".to_string()),
            IntErrorKind::PosOverflow => Err("".to_string()),
            IntErrorKind::NegOverflow => Err("".to_string()),
            IntErrorKind::Zero => Err("".to_string()),
            //IntErrorKind::NotAPowerOfTwo => {Err("".to_string())}
            _ => Err(format!("{}", e)),
        },
    }
}

fn test() -> io::Result<()>{


    Ok(())
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut user_input = String::new();

    println!("Enter a number");

    let _ = io::stdout().flush();

    let result = parse_integer_from_string(&mut user_input);

    match result {
        Ok(n) => println!("The number is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
