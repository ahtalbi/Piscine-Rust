use core::f64;
use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", name[0..1].to_string()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let function = self.flags.get(input).unwrap();
        let call = function(argv[0], argv[1]);
        match call {
            Ok(res) => Ok(res),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let var1: f64;
    let var2: f64;
    match a.parse::<f64>() {
        Ok(num1) => var1 = num1,
        Err(e) => return Err(e),
    };
    match b.parse::<f64>() {
        Ok(num2) => var2 = num2,
        Err(e) => return Err(e),
    }
    Ok((var1 / var2).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let var1: f64;
    let var2: f64;
    match a.parse::<f64>() {
        Ok(num1) => var1 = num1,
        Err(e) => return Err(e),
    };
    match b.parse::<f64>() {
        Ok(num2) => var2 = num2,
        Err(e) => return Err(e),
    }
    Ok((var1 % var2).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut handler = FlagsHandler { flags: HashMap::new() };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag(d, div);
        handler.add_flag(r, rem);

        println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

        println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

        println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

        println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
    }
}
