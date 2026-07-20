pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        return s.strip_prefix(prefix);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", delete_prefix("ab", "abcdefghijklmnop"));
        println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
    }
}
