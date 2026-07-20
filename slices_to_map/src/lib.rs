use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> where T: Eq + Hash {
    let mut map = HashMap::new();
    
    for i in 0..keys.len().min(values.len()) {
        map.insert(&keys[i], &values[i]);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        println!("{:?}", slices_to_map(&keys, &values));
    }
}