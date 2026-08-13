trait Animal {
    fn make_sound(&self);
    fn set_age(&mut self, age: u8) {
        println!("此功能暂不支持")
    }
    fn get_age(&self) -> u8 {
        println!("此功能暂不支持");
        0
    }
}

#[derive(Default)]
struct Dog {
    age: u8,

}

#[derive(Default)]
struct Cat {
    age: u8,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("大狗大狗，叫叫叫！")
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("卡拉比丘喵")
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut my_dog = Dog::default();


}
