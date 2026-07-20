use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort();
    let m = v.len() / 2;
    if v.len() % 2 == 0 {
        (v[m - 1] + v[m]) / 2
    } else {
        v[m]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut mode = 0;
    let mut max = 0;

    for n in list{
        map.entry(*n).and_modify(|n| *n +=1 ).or_insert(1);
    } 

    for (key, val) in map{
        if val > max{
            max = val;
            mode = key;
        }
    }
    return mode

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        
        println!("mean {}", mean(&v));
        println!("median {}", median(&v));
        println!("mode {}", mode(&v));
    }
}
