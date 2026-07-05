// ive learned that i need to remember rem_euclid
pub fn rotate(input: &str, key: i8) -> String {
    let mut res : String = String::new();
    let v : Vec<char> = ('a'..='z').collect();
    for (i, c) in input.chars().enumerate() {
        if c.is_alphabetic() {
            let diff = if c.is_uppercase() {
                c as i32 - 'A' as i32
            } else {
                c as i32 - 'a' as i32
            };

            let chr = if c.is_lowercase() {
                v[((diff + (key as i32)).rem_euclid(26)) as usize]
            } else {
                v[((diff + (key as i32)).rem_euclid(26)) as usize].to_ascii_uppercase()
            };
            res.push(chr);
        } else {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("The letter \"a\" becomes: {}", rotate("a", 26));
        println!("The letter \"m\" becomes: {}", rotate("m", 0));
        println!("The letter \"m\" becomes: {}", rotate("m", 13));
        println!("The letter \"a\" becomes: {}", rotate("a", 15));
        println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
        println!(
            "The decoded message is: {}",
            rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
        );
        println!(
            "The decoded message is: {}",
            rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
        );
        println!(
            "Your cypher wil be: {}",
            rotate("Testing with numbers 1 2 3", 4)
        );
        println!("Your cypher wil be: {}", rotate("Testing", -14));
        println!("The letter \"a\" becomes: {}", rotate("a", -1));
    }
}
 