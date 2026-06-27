pub fn to_url(s: &str) -> String {
    s.replace(' ', "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
        assert_eq!(to_url(s), "Hello,%20world!");
    }
}
