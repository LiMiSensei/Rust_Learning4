struct TaxCalculator {
    calculation: Box<dyn Fn(i32) -> i32>,
}

impl TaxCalculator {
    fn new(calculation: Box<dyn Fn(i32) -> i32>) -> Self {
        let calculator = Self { calculation };
        calculator
    }

    fn calculate(&self, amount: i32) -> i32 {
        (self.calculation)(amount)
    }
}

#[allow(dead_code)]
pub fn main() {
    let vat_calculator = TaxCalculator::new(Box::new(|x| x + 1));
}
