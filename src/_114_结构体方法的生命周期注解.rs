struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

//'a 使用结构体的生命周期
//‘b，’c 使用结构体字段的生命周期
impl<'a, 'b> MyStruct<'a, 'b> {
    fn get_data1(&self) -> &'a str {
        self.data1
    }
    fn set_data(&mut self, s1: &'a str, s2: &'b str) {
        self.data1 = s1;
        self.data2 = s2;
    }

    fn get_longest<'c>(&'c self, s: &'c str) -> &str {
        let longest_self = if self.data1.len() > self.data2.len() {
            self.data1
        } else {
            self.data2
        };

        if longest_self.len() > s.len() {
            longest_self
        } else {
            s
        }
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let res;
    {
        let my_struct = MyStruct {
            data1: "Hello",
            data2: "World",
        };

        res = my_struct.get_data1()
    };

    println!("{}", res);

    let mut my_struct = MyStruct {
        data1: "Hello",
        data2: "World",
    };
    let s1 = "Good";
    let s2 = "Morning";
    my_struct.set_data(s1, s2);
}
