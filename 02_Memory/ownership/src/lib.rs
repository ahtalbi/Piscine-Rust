pub fn first_subword(mut s: String) -> String {
    for (i, c) in s.chars().enumerate() {
        if i == 0 {continue;}
        if c.is_ascii_uppercase() || c == '_' {
            return s[..i].to_string();
        }
    }
    s.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld";
        let s2 = "snake_case";
        let s3 = "CamelCase";
        let s4 = "just";

        assert_eq!(first_subword(s1.to_owned()), "hello");
        assert_eq!(first_subword(s2.to_owned()), "snake");
        assert_eq!(first_subword(s3.to_owned()), "Camel");
        assert_eq!(first_subword(s4.to_owned()), "just");
    }
}
