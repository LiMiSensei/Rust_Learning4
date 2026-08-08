#[allow(dead_code)]
pub fn main() {
   let my_array = [2.5,4.0,3.8];
   let my_array = [1,2,3_u8,4,5];
   let my_array = [0;10];
   let my_array = ['+','-'];
   let my_array = [0_u8;1024];
    
    let mut sum = 0_u8;
    for e in my_array{
        sum += e;
    }
    println!("{}",sum);
}
