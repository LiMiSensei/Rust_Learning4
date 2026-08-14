#[allow(dead_code)]
pub fn main() {

    let mut print;
    {
        let mut x = 1;
        // move关键字将变量的所有权转移给闭包
        print = move || {
            x += 1;
            println!("{}", x);
        };
    }

    print();
}
