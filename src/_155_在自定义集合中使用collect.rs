#[derive(Debug)]
struct Transaction{
    amount:f64,
    desscription:String,
}
#[allow(dead_code)]
pub fn main() {
    let transaction = vec![
        Transaction{
            amount: 100.0,
            desscription: "G".to_string(),
        },
        Transaction{
            amount: 234.0,
            desscription: "H".to_string(),
        },
        Transaction{
            amount: 13.0,
            desscription: "C".to_string(),
        },
        Transaction{
            amount: 134.0,
            desscription: "L".to_string(),
        },
    ];

    //let summary :(f64,usize) = transaction.into_iter().collect();

}
