enum LightState {
    On { brightness: u8 },
    Off,
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let nulb = LightState::On { brightness: 10 };
    match nulb {
        LightState::On { brightness: 180 } => {}
        LightState::Off => {}
        _ => {}
    };
}
