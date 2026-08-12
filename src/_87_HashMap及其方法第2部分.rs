use std::collections::HashMap;

fn count_world(text: &str) -> HashMap<&str, u32> {
    let mut worl_count = HashMap::new();
    let processed_text = text.chars().filter(|c| c.is_alphanumeric()); //过滤方法

    for world in text.split_whitespace() {
        worl_count.entry(world).and_modify(|e| *e += 1).or_insert(1);
    }
    worl_count
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let mut fruit_prices = HashMap::new();

    fruit_prices.insert(String::from("apple"), 50);
    fruit_prices.insert(String::from("banana"), 123);
    fruit_prices.insert(String::from("cherry"), 32);

    match fruit_prices.get("apple") {
        Some(i) => println!("apple: {}", i),
        None => println!("apple not found"),
    }

    if fruit_prices.contains_key("banana") {
        let counnt = fruit_prices.get_mut("banana").unwrap();
        *counnt += 1;
    } else {
        fruit_prices.insert("banana".to_string(), 1);
    }

    let mut fruits = HashMap::new();
    *fruits.entry(String::from("banana")).or_insert(0) += 1;
    *fruits.entry(String::from("banana")).or_insert_with(|| 2);
    *fruits
        .entry(String::from("banana"))
        .and_modify(|x| *x += 2)
        .or_insert(1);

    println!("{:?}", fruits);

    //=======================
    let test = "this is a ssample text";
    let world = count_world(test);

    for (k, v) in &world {
        println!("{}: {}", k, v);
    }
}
