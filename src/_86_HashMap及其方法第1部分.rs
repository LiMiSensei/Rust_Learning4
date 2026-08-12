use std::collections::HashMap;

#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut fruit_prices = HashMap::new();

    fruit_prices.insert(String::from("apple"), 50);
    fruit_prices.insert(String::from("banana"), 123);
    fruit_prices.insert(String::from("cherry"), 32);

    match fruit_prices.get("apple") {
        Some(i) => println!("apple: {}", i),
        None => println!("apple not found")
    }

    if fruit_prices.contains_key("banana") {
        let counnt = fruit_prices.get_mut("banana").unwrap();
        *counnt += 1;
    }else{
        fruit_prices.insert("banana".to_string(),1);
    }

    let mut fruits = HashMap::new();
    *fruits.entry(String::from("banana")).or_insert(0) += 1;
    *fruits.entry(String::from("banana")).or_insert_with(||{2});

    println!("{:?}", fruits);

}
