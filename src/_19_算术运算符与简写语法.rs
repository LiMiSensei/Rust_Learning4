#[allow(dead_code)]
pub fn main() {
    let arg1: u8 = 5;
    let arg2: u8 = 10;
    let sum = arg1 / arg2;

    let patterns = [1, 2, 3, 4, 5, 6, 7];
    let mut i = 0;
    loop {
        control_light(patterns[i % patterns.len()]);
        //delay(1000);
        i = i + 1;
        
        break
    }

    let mut a = 5;
    a += 5;
    a *= 5;
}

fn control_light(p0: i32) {}
