use std::ops::RangeBounds;
fn combine<T,U>(a:T,b:U) -> (T,U ){
    (a,b)
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main(){
    let t1 = combine(1,2);


}
