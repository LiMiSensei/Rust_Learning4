#[derive(Debug,Default)]
struct Size(i32,i32,i32);
#[derive(Debug,Default)]
struct Point(i32,i32,i32);

struct Person{
    name:String,
    age:u32,
}

fn refactor_point(point: Point){
    println!("{},{},{}",point.0,point.1,point.2);
}

fn uppdate_person_age(person:&mut Person,new_age:u8){

}
#[allow(dead_code)]
pub fn main() {


}
