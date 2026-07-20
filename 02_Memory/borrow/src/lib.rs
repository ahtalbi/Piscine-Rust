pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "hello";
	    let s1 = "camelCase".to_string();
	    let s2 = "olá!";

	    assert_eq!(str_len(s), 5);
	    assert_eq!(str_len(&s1), 9);
	    assert_eq!(str_len(s2), 4);
    }
}
