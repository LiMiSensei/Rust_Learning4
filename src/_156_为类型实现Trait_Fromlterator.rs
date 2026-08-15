use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug)]
struct Transaction {
    amount: f64,
    desscription: String,
}

impl FromIterator<Transaction> for (f64, usize) {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Transaction>,
    {
        let mut total_amount = 0.0;
        let mut total_transcations = 0;
        for transcation in iter {
            total_amount += transcation.amount;
            total_transcations += 1;
        }
        (total_amount, total_transcations)
    }
}
#[allow(dead_code)]
pub fn main() {
    let transaction = vec![
        Transaction {
            amount: 100.0,
            desscription: "G".to_string(),
        },
        Transaction {
            amount: 234.0,
            desscription: "H".to_string(),
        },
        Transaction {
            amount: 13.0,
            desscription: "C".to_string(),
        },
        Transaction {
            amount: 134.0,
            desscription: "L".to_string(),
        },
    ];

    //为自定义类型实现 from iterator特质
    let summary: (f64, usize) = transaction.into_iter().collect();

    //let transaction_map: HashMap<String,f64> = transaction.map().collect();
    
    println!("{:?}", summary);
}
