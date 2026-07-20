pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rev_str("Hello, world!"), "!dlrow ,olleH");
        assert_eq!(rev_str("Hello, my name is Roman"), "namoR si eman ym ,olleH");
        assert_eq!(rev_str("I have a nice car!"), "!rac ecin a evah I");
        assert_eq!(rev_str("How old are You"), "uoY era dlo woH");
        assert_eq!(rev_str("ex: this is an example água"), "augá elpmaxe na si siht :xe");
    }
}
