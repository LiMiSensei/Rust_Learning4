fn call_fn_mut<F: FnMut() -> i32>(mut f: F) {
    println!("{}", f());
}

fn call_fn_once<F: FnOnce() -> i32>(mut f: F) {
    println!("{}", f());
}
fn call_fn<F: Fn() -> i32>(mut f: F) {
    println!("{}", f());
}
#[allow(dead_code)]
pub fn main() {
    let x = 5;
    let closure = || x * 2;

    call_fn_mut(&closure);
}
