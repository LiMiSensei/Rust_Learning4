#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
pub fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    //未被执行
    let mut v1 = numbers.iter_mut().map(|num| *num += 1);
    println!("{:?}", v1);
    let v2: Vec<_> = v1.collect();
    println!("{:?}", numbers);

    let mut numbers = vec![1, 2, 3, 4, 5];

    //直接执行
    let _ = numbers.iter_mut().for_each(|x| *x += 1);
    println!("{:?}", numbers);

    //===============================

    let data = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];

    let points: Vec<Point> = data.into_iter().map(|(x, y)| Point { x, y }).collect();

    println!("{:?}", points);
}
