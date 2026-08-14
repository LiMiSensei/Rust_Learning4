#[allow(dead_code)]
pub fn main() {
    let array = [1, 2, 3, 4, 5];

    for i in array.iter() {}

    for i in array.into_iter() {}

    println!("{:?}", array);


    let array = ["123","312","231"];

    for i in array.iter() {}

    for i in array.into_iter() {}

    println!("{:?}", array)
}
