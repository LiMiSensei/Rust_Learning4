use std::fmt::{Display, Formatter};

struct Dog{
    weight:u8,
    age:u8,
    name:String,
}

impl Display for Dog{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")?;
        write!(f, "{}", self.age)?;
        write!(f, "{}", self.weight)?;
        write!(f, "{}", self.name)?;

        Ok(())
    }
}
#[allow(dead_code)] //ref 是引用绑定
pub fn main() {


}
