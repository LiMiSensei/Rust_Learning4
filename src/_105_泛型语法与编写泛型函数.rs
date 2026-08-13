use std::ops::RangeBounds;

fn find_max_int(v: &[i32]) -> Option<i32> {
    if v.is_empty() {
        return None;
    }
    let mut max = v[0];

    for n in v {
        if *n > max {
            max = *n
        }
    }

    Some(max)
}

fn find_max<I>(iter: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::Item: PartialOrd,
{
    let mut iter = iter.into_iter();
    let mut max = iter.next()?;

    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let array = [1,2,3,4,5];

    let arr_float = [1.0,2.3,1.2,4.4,5.5];



}
