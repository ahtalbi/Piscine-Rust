use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let v: Vec<char> = ('a'..='z').collect();
    let mut h: HashMap::<char, bool> = v.iter().map(|x| (*x, false)).collect();
    for c in s.chars() {
        if c.is_alphabetic() {
            if c.is_ascii_uppercase() {
                h.insert(c.to_ascii_lowercase(), true);
            } else {
                h.insert(c, true);
            }
        }
    }

    for b in h.keys() {
        if *h.get(&b).unwrap() == false {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            is_pangram("the quick brown fox jumps over the lazy dog!")
        );
        println!("{}", is_pangram("this is not a pangram!"));
    }
}
