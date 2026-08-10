struct Mystruct {}

impl Mystruct {
    fn new()->Self{
        Mystruct{}
    }
    fn f1(&self) {}

    fn f2(p: &Mystruct) {}

    fn f3(i:i32)->i32{
        1//关联函数要使用双冒号
    }
}
#[allow(dead_code)]
pub fn main() {
    let p = Mystruct {};

    p.f1();
    Mystruct::f2(&p);
    let my = Mystruct::new();
    Mystruct::f3(1);//这就是关联函数
}
