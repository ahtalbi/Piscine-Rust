use std::collections::HashMap;

use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false; }

    let mut map1: HashMap<char, usize> = HashMap::new();
    let mut map2: HashMap<char, usize> = HashMap::new();

    for c in s1.chars() {
        *map1.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *map2.entry(c).or_insert(0) += 1;
    }

    for (key, val) in &map1 {
        if map2.get(key) != Some(val) {
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
        let word = "thought";
        let word1 = "thougth";

        println!(
            "Is {:?} a permutation of {:?}? = {}",
            word,
            word1,
            is_permutation(word, word1)
        );
    }
}
