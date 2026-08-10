#[allow(dead_code)]
pub fn main() {
    let tuple1 = (2, 3, 5);
    let tuple2 = (0, 3, 4);
    let tuple3 = (1, 2, 3);

    if tuple1 < tuple2 {
        println!("tuple1 is less than tuple2");
    } else {
        println!("tuple1 is greater than tuple2");
    }

    let tuple4 = (1, "x");
    let tuple5 = (2, "y");
    let tuple6 = (3, "z");

    if tuple4 > tuple5 {
        println!("tuple4 is greater than tuple5");
    } else {
        println!("tuple5 is greater than tuple5");
    }

    let rcvd_data = (5, "hello", 8);

    match rcvd_data {
        (a, b, c) if a > 0 && c < 10 => println!("rcvd_data is less than rcvd_data"),
        _ => println!("rcvd_data is greater than rcvd_data"),
    }
}
