struct TaxCalculator<F: Fn(f32) -> f32> {
    calculation: F,
}

impl<F: Fn(f32) -> f32> TaxCalculator<F> {
    fn new(calculation: F) -> TaxCalculator<F> {
        let calculator = Self { calculation };
        calculator
    }

    fn calculate(&self, amount: f32) -> f32 {
        (self.calculation)(amount)
    }
}

#[allow(dead_code)]
pub fn main() {

    let vat_calculator = TaxCalculator::new(Box::new(|x| x + 1.0));
}
