use std::rc::Rc;

#[allow(dead_code)]
pub fn main() {
    let strong_re_1 = Rc::new(String::from("hello"));

    println!("strong_re: {}", Rc::strong_count(&strong_re_1));
    println!("strong_re: {}", Rc::weak_count(&strong_re_1));

    //这是强引用
    let strong_rc_2 = strong_re_1.clone();
    println!("strong_re: {}", Rc::strong_count(&strong_re_1));
    println!("strong_re: {}", Rc::weak_count(&strong_re_1));

    let weak_ref_1 = Rc::downgrade(&strong_re_1);
    println!("strong_re: {}", Rc::strong_count(&strong_re_1));
    println!("strong_re: {}", Rc::weak_count(&strong_re_1));

    let strong_rc_3 = weak_ref_1.upgrade();
    println!("strong_re: {}", Rc::strong_count(&strong_re_1));
    println!("strong_re: {}", Rc::weak_count(&strong_re_1));

    println!("{}",strong_re_1);
    println!("{:?}",weak_ref_1);

    if let Some(upgraded) = weak_ref_1.upgrade(){
        println!("{}",upgraded);
    }else{
        println!("no upgrade");
    }

    //=================================================
    let mut strong_rc_test = Rc::new(5);
    let mut strong_rc_test = strong_rc_test.clone();

    if let Some(r) = Rc::get_mut(&mut strong_rc_test){
        *r = 10;
        println!("{}",r)
    }else {
        println!("no such rc");
    }
    println!("{:?}",strong_rc_test);
    //=================================================


}
