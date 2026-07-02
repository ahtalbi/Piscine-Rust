use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: "-".to_string() + &name[0..1],
            long_hand: name.to_string(),
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
        self.flags.insert(flag.long_hand.clone(), func);
        self.flags.insert(flag.short_hand.clone(), func);
    }
    
    pub fn exec_func(&self, _input: &str, argv: &[&str]) -> Result<String, String> {
        let func = self.flags.get(_input).ok_or("Unknown flag")?;

        let a = argv.get(1).ok_or("invalid float literal")?;
        let b = argv.get(2).ok_or("invalid float literal")?;

        Ok(func(a, b))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;

    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;

    Ok((a % b).to_string())
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
