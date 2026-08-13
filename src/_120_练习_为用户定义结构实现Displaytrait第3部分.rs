struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy + PartialOrd> Rectangle<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {}
