use std::fmt::format;

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut v = Vec::with_capacity(20);

    for i in 1..=4{
        v.push(format!("i={}", i));
    }

    let vector_base_addr_in_heap_1 = v.as_ptr();
    println!("{}",v.capacity());

    v.push("666".to_string());
    let vector_base_addr_in_heap_2 = v.as_ptr();
    println!("{}",v.capacity());

    let v = vec!["sun".to_string(),"sun".to_string(),"sun".to_string()];

    let s = &v[0];//实际上String不实现复制，所有权被移动！
    println!("{:?}",v);

    //安全的访问
    let mut vec = vec![1,2,3];
    
    let val_ref = vec.get(1);
    
    if let Some(val) = val_ref{
        println!("{}",val)
    }
    
    
    let val_mut_ref = vec.get_mut(2);
    
    if let Some(val) = val_mut_ref{
        *val *= 10;
    }
    
    println!("{:?}",vec);

}
