#[allow(dead_code)]
pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for number in numbers.iter().map(|&n| n * 2).filter(|&n| n > 5) {
        println!("{}", number);
    }

    numbers
        .iter()
        .map(|&n| n * 2)
        .filter(|&n| n > 5)
        .for_each(|number| println!("{}", number));
}
