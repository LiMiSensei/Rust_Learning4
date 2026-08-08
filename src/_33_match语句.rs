#[allow(dead_code)]
pub fn main() {
    //=====================================
    let x = 5;
    match x {
        101 | 102 => println!("x is 101 or 102"),
        0..100 => println!("0-100"),
        _ => println!(""),
    }

    //=====================================
    let point = (3, -7);

    match point {
        (x, y) if y < 0 => println!("y小于0"),
        (0, 0) => println!("0，0"),

        _ => println!("all")
    }

    //=====================================
    let array = [1,2,3,4];
    let invaid_array = match array {
        [n,_,_,_] if n < 0 => true,
        [n,n,_,_] if n > 0 => true,
        _ => false
    };
    //matches！  用于bool的判断的守卫模式
    let invaid_array = matches!(invaid_array,[n,_,_,_] if n < 0);



}
