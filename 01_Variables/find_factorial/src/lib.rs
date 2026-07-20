pub fn factorial(num: u64) -> u64 {
    if num == 0 {return 1;}
    let mut res = 1;
    for i in 1..num+1 {
        res = res * i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(19), 121645100408832000);
    }
}
