#[allow(dead_code)]
pub fn main() {
    let y = 1;
    //let c1 = |x| x + y; //即使两个闭包相同，他们也是两个不同的类型
    //let c2 = |x| x + y;

    let c3: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + y);
    let c4: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + y);

    let vec_closures = vec![c3,c4];

    println!("{:?}", vec_closures[0](1));
}
