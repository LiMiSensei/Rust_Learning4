struct Pair<T, U> {
    first: T,
    second: U,
}
impl<T> Pair<T, T> {

}

impl<T:Copy> Pair<T, T> {
    
}

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {

}
