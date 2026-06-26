pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(divide(9, 4), (2, 1));
    }
}
