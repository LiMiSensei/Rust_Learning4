#[allow(dead_code)]
pub fn main() {
    let the_data = (
        "Monday".to_string(),
        25,
        "25".to_string(),
        "June".to_string(),
        "2023".to_string()
        );

    match the_data {
        (ref day,..) if day =="Sunday" => {
            println!("Sunday");
        }
        _=> println!("Sunday"),
    }

    println!("{:?}", the_data);

}
