use std::collections::HashMap;

fn give_score(values_map: &mut HashMap::<char, u64>, v :Vec<char>, value: u64) {
    for c in v {
        values_map.insert(c, value);
    }
}

pub fn score(s: &str) -> u64 {
    let mut values : HashMap::<char, u64> = HashMap::new();
    // init
    let v1  = vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
    let v2  = vec!['D', 'G'];
    let v3  = vec!['B', 'C', 'M', 'P'];
    let v4  = vec!['F', 'H', 'V', 'W', 'Y'];
    let v5  = vec!['K'];
    let v8  = vec!['J', 'X'];
    let v10 = vec!['Q', 'Z'];
    // put in values
    give_score(&mut values, v1 , 1 );
    give_score(&mut values, v2 , 2 );
    give_score(&mut values, v3 , 3 );
    give_score(&mut values, v4 , 4 );
    give_score(&mut values, v5 , 5 );
    give_score(&mut values, v8 , 8 );
    give_score(&mut values, v10, 10);
    
    let mut res : u64 = 0;
    for c in s.chars() {
        if c.is_ascii_lowercase() {
            if values.contains_key(&c.to_ascii_uppercase()) {
                res = res + values.get(&c.to_ascii_uppercase()).unwrap();
            }
            continue;
        }
        if values.contains_key(&c) {
            res = res + values.get(&c).unwrap();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", score("a"));
        println!("{}", score("ã ê Á?"));
        println!("{}", score("ThiS is A Test"));
    }
}
