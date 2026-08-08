use std::io;

#[allow(dead_code)]
pub fn main() {}

fn read_rom_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn parse_string_as_u32(input: String) -> u32{
    let total_srconds: u32 = input
        .trim().parse().unwrap();

    return total_srconds;
}