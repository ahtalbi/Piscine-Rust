fn have_multiplayer(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if i * i == n {
            return true;
        }
        i += 1;
    }
    false
}

fn first_fifty_even_square() -> Vec<i32> {
    let mut res : Vec<i32> = Vec::new();
    let mut i = 4;
    while res.len() < 50 {
        if have_multiplayer(i) {
            res.push(i);
        }
        i += 2;
    }
    res
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
