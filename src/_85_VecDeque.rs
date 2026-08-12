use std::collections::VecDeque;

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut vd = VecDeque::with_capacity(6);
    vd.push_back(1);
    vd.push_back(1);
    vd.push_back(1);
    vd.push_back(1);
    vd.push_back(1);

    let vd_slice:&mut [i32] = vd.make_contiguous();

    let (front,back) = vd.as_slices();


}
