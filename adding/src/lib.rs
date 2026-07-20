pub fn add_curry(bn: i64) -> impl Fn(i64) -> i64 {
    move |n| n + bn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add10 = add_curry(-10);
        let add20 = add_curry(2066);
        let add30 = add_curry(300000);

        println!("{}", add10(5));
        println!("{}", add20(195));
        println!("{}", add30(5696));
    }
}
