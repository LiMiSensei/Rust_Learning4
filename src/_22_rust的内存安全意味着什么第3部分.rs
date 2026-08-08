use std::thread;


#[allow(dead_code)]
pub fn main() {
    let data = 42;
    let handlel = thread::spawn(|| {
        println!("1");
    });

    let handlel2 = thread::spawn(|| {
        println!("2");
    });

    handlel.join().unwrap();
    handlel2.join().unwrap();

}
