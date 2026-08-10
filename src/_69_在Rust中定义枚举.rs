#[derive(PartialEq)]  // ✅ 加上这个
struct Point{

}
#[derive(PartialEq)]  // ✅ 加上这个
enum CarStatus1 {
    MovingUp { speed: u32, x: i32, y: i32 },
    MovingDown { speed: u32 },
    NotMoving(Point),
    NotWorking,
}

enum GameState {
    Restart,
    End,
    Pause,
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut current_car_status = CarStatus1::NotMoving(Point{});
    current_car_status = CarStatus1::MovingUp { speed: 100, x: 1, y: 1 };


    if current_car_status == CarStatus1::NotWorking{
        println!("Not working");
    }

}
