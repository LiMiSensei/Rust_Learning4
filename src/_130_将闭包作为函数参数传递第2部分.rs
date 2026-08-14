fn apply<F>(f:F, arg:i32) -> i32
where F: Fn(i32)->i32{
    f(arg)
}
#[allow(dead_code)]
pub fn main() {
    let y = 2;
    let multiply = |x| x * y;

    let result = apply(multiply, 4);
    println!("result: {}", result);
}
