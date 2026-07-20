fn first_fifty_even_square() -> Vec<i32> {
    (2..)
        .step_by(2)
        .map(|n| n * n)
        .take(50)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    println!("Hello, world!");
	    let v1 = first_fifty_even_square();

	    println!("All elements in {:?}, len = {}", v1, v1.len());
    }
}
