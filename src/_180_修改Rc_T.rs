use std::rc::Rc;

#[allow(dead_code)]
pub fn main() {
    let mut data = Box::new(50);

    println!("{}", *data); // Box<T>实现了Dref

    *data += 49;

    println!("{}", *data); // Box<T>实现了DrefMut特质

    //=============================================

    let mut data = Rc::new(50);

    println!("{}", *data); // Rc<T>实现了Dref

    //*data += 49;

    println!("{}", *data); // Rc<T>未实现了DrefMut特质

    //=============================================
    //只有一个强引用，0个弱引用才能使用

    let mut p = Rc::new(6);
    if let Some(v) = Rc::get_mut(&mut p) {
        *v = 100;
    };

    println!("{}", *p);
}
