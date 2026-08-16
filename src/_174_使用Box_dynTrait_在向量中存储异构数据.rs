

trait InterestCalculator {
    fn calculate_interest(&self) -> f64;
}
struct SavingsAccount {
    balance: f64,
}

impl InterestCalculator for SavingsAccount {
    fn calculate_interest(&self) -> f64 {
        self.balance * 0.03
    }
}

struct FixedDeposit {
    amount: f64,
    duration: f64,
}

impl InterestCalculator for FixedDeposit {
    fn calculate_interest(&self) -> f64 {
        self.amount * self.duration
    }
}

fn desplay_iterest(account: Box<dyn InterestCalculator>) {
    println!("InterestCalculator{}", account.calculate_interest());
}
#[allow(dead_code)]
pub fn main() {
    let sacings = Box::new(SavingsAccount { balance: 2.0 });
    let fixed = Box::new(FixedDeposit { amount: 100.0, duration: 100.0, });

    let v: Vec<Box<dyn InterestCalculator>> = vec![sacings,fixed];

    let sacings2 = SavingsAccount { balance: 2.0 };
    let fixed2 = FixedDeposit { amount: 100.0, duration: 100.0, };

    let v2: Vec<&dyn InterestCalculator> = vec![&sacings2,&fixed2];


}
