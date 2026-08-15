use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    //collect 会有个迭代器并且会消耗这个迭代器
    //=========================================================
    let numbers = vec![12, 3, 4, 5, 6];

    let numbber_string = numbers
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", numbber_string);

    //=========================================================
    let fruit_basket = vec![("A", 1), ("B", 2), ("C", 3), ("D", 4)];

    let friuts_map: HashMap<_, _> = fruit_basket.into_iter().collect();

    println!("{:?}", friuts_map);
    //=========================================================
    let chars = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    let word: String = chars.into_iter().collect();

    println!("{:?}", word);
    //=========================================================
}
