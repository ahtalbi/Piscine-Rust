pub fn stars(n: u32) -> String {
    "*".repeat(2_i32.pow(n).try_into().unwrap()).to_string()
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
