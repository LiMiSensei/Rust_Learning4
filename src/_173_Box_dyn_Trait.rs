use std::time::Duration;

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
    desplay_iterest(sacings);
    let fixed = Box::new(FixedDeposit {
        amount: 100.0,
        duration: 100.0,
    });
    desplay_iterest(fixed);
}
