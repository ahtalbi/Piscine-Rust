pub fn stars(n: u32) -> String {
    "*".repeat(((2 as i32).pow(n)) as usize).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", stars(1));
        println!("{}", stars(4));
        println!("{}", stars(5));
    }
}
