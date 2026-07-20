use adding::add_curry;

pub fn twice(f: impl Fn(i64) -> i64) -> impl Fn(i64) -> i64 {
    move |c| f(f(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add10 = add_curry(10);
        let value = twice(add10);
        println!("The value is {}", value(7));

        let add20 = add_curry(20);
        let value = twice(add20);
        println!("The value is {}", value(7));

        let add30 = add_curry(30);
        let value = twice(add30);
        println!("The value is {}", value(7));

        let neg = add_curry(-32);
        let value = twice(neg);
        println!("The value is {}", value(7));
    }
}
