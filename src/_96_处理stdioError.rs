use std::fs;

fn rename_file(form: &str, to: &str) -> Result<(), std::io::Error> {
    match fs::rename(form, to) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
fn rename_file1(form: &str, to: &str) -> std::io::Result<()> {
    fs::rename(form, to)?;
    Ok(())
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {
    let res = rename_file("logo.txt", "output.txt");
    let res2 = rename_file1("logo.txt", "output.txt");
    if res.is_err() {
        println!("出错了")
    }
}
