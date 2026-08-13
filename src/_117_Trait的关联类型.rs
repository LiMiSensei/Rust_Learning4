trait Animal {
    fn make_sound(&self);

    type Weight;
    fn set_weight(&mut self, weight: Self::Weight);
    fn get_weight(&self) -> Self::Weight;
    fn set_age(&mut self, age: u8);
    fn get_age(&self) -> u8;
}

#[derive(Default)]
struct Dog {
    age: u8,
    welght: u8,
}

#[derive(Default)]
struct Cat {
    age: u8,
    welght: f32,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("大狗大狗，叫叫叫！")
    }

    type Weight = u8;

    fn set_weight(&mut self, weight: Self::Weight) {
        self.welght = weight
    }

    fn get_weight(&self) -> Self::Weight {
        self.welght
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("卡拉比丘喵")
    }

    type Weight = f32;

    fn set_weight(&mut self, weight: Self::Weight) {
        self.welght = weight
    }

    fn get_weight(&self) -> Self::Weight {
        self.welght
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut my_dog = Dog::default();
}
