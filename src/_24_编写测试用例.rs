use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    let currency_symbol = '$';

    println!("贷款金额{}", currency_symbol);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let princupal: f64 = input.trim().parse().expect("Please type a number!");
    input.clear();

    println!("年利率 %");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let annual_tate: f64 = input.trim().parse().expect("Please type a number!");
    input.clear();

    println!("期限 /月");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let moths: u32 = input.trim().parse().expect("Please type a number!");
    input.clear();

    let emi: f64 = calculate_emi(princupal, annual_tate, moths);
    println!("月供：{}", emi);
}

fn calculate_emi(p0: f64, p1: f64, p2: u32) -> f64 {
    (p0 + (p1 * p0 * 0.01)) / (p2 as f64)
}



#[cfg(test)]
mod tests{
    use std::io;

    fn get_user_inpuut() -> u32{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let annual_tate: u32 = input.trim().parse().expect("Please type a number!");
        input.clear();
        annual_tate
    }

    fn conver_seccnds_to24hr_format(total_seconds: u32) -> String {
        if total_seconds > 86399 {
            panic!("Please check your seconds!");
        }

        let hours = total_seconds / 3600;
        let remaining_seconds = total_seconds % 3600;
        let minutes = remaining_seconds / 60;
        let seconds = remaining_seconds % 60;

        let format_24hr = format!(
            "Time in 24hr format is {:02}:{:02}:{:02}",
            hours, minutes, seconds
        );
        return format_24hr;
    }

}
