pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::new();
    
    for i in 0..arr.len() {
        let mut tmp = arr.clone();
        tmp[i] = 1;
        res.push(tmp.iter().product());
    }
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        println!("{:?}", output);
    }
}
