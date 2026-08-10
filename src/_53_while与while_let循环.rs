#[allow(dead_code)]
pub fn main() {
    let mut number = [1, 2, 3, 4, 5, 6, 7];
    let i = 0;
    let mut interator = number.iter();

    while let Some(number) = interator.next() {
        if number % 2 == 0 {
            println!("1")
        } else {
            println!("2")
        }
    }


    let mut number = [1, 2, 3, 4, 5, 6, 7];
    let message = 'outer: loop {
        for n in number {
            if n < 0 {
                break 'outer "Invalid array"
            }
        }

        break 'outer "Valid array"
    };

    print!("{}", message);
}
