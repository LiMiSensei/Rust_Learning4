use std::cell::{RefCell, RefMut};
use std::ops::AddAssign;
use std::rc::Rc;

#[allow(dead_code)]
pub fn main() {
    let data = Rc::new(RefCell::new(100));

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    //let mut t1:i32 = *owner1.borrow_mut(); //这是非法分，可变引用正在被持有
    //t1 += 10;
    *owner1.borrow_mut() += 10;

    //let t2: RefMut<i32> = (*owner1).borrow_mut();

    println!("{}",data.borrow())
}

