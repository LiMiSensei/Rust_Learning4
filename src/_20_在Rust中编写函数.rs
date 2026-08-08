mod module_2 {
    pub fn convert_kg_to_grams(in_kg: f64) {
        println!("TODO{}", in_kg)
    }
}

pub fn convert_kg_to_grams(in_kg: f64) -> f64 {
    println!("TODO{}", in_kg);
    in_kg
}
pub fn convert_kg_to_string<'a>(first:&'a str,second:&'a str) -> &'a str {
    "666"
}
mod modle_1 {
    pub fn convert_kg_to_grams(in_kg: i32) {
        println!("TODO{}", in_kg)
    }
}

#[allow(dead_code)]
pub fn main() {
    module_2::convert_kg_to_grams(4 as f64);
    modle_1::convert_kg_to_grams(4);

    let value = convert_kg_to_grams(12.0);
}
