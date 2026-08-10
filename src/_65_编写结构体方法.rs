struct Mystruct {}

impl Mystruct {
    fn f1(&self) {}

    fn f2(p: &Mystruct) {}
}
#[allow(dead_code)]
pub fn main() {
    let p = Mystruct {};

    p.f1();
    Mystruct::f2(&p);
}
