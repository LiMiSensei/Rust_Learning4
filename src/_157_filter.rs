#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
pub fn main() {
    let number = vec![1, 2, 3, 4, 5, 6];

    let event = number.iter().filter(|x| **x % 2 == 0);

    for num in event {
        println!("{}", num);
    }

    let number = vec![
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
    ];

    let less_than_four = number.iter().filter(|point| point.x < 5);

    for num in less_than_four {
        println!("{:?}", num);
    }
}
