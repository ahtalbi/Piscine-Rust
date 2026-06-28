pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5];

        println!("The sum of the elements in {:?} is {}", a, sum(&a));
        println!("The sum of the elements in {:?} is {}", b, sum(&b));
        println!(
            "Array of {} elements filled with 10 = {:?}",
            thirtytwo_tens().len(),
            thirtytwo_tens()
        );
    }
}
