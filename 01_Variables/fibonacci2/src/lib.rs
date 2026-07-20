pub fn fibonacci(n: u32) -> u32 {
    if n==0{return 0;}
    if n <= 2 { return 1; }

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    for _ in 2..n+1 {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(22), 17711);
    }
}
