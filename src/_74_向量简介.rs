

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    let v = vec![1,2,3,4,5];


    for &i in v.iter() {
        println!("{}",i)
    }

    let arr = [1,2,3,4];

    let vec1 = arr.to_vec();
    let vec2 = Vec::from(arr);
    let vec3 = Vec::from([1,2,3,4]);
    let vec4 = Vec::from([10;5]);

    println!("{:?}",vec4);


}
