use std::mem;

pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    println!("{:?}", s);
    let splitted = s.split(' ');
    let mut res : Vec<Box<u32>> = Vec::new();
    for v in splitted {
        if v.ends_with('k') {
            let num = &v[..v.len() - 1];
            let n = (num.parse::<f32>().unwrap() * 1000.0) as u32;
            res.push(Box::new(n));
        }
    }
    println!("{:?}", res);
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res : Vec<u32> = Vec::new();
    for i in a {
        res.push(*i)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "5.5k 8.9k 32".to_owned();

        let boxed = parse_into_boxed(s);
        println!("Element value: {:?}", boxed[0]);
        println!("Element size: {:?} bytes", mem::size_of_val(&boxed[0]));

        let unboxed = into_unboxed(boxed);
        println!("Element value: {:?}", unboxed[0]);
        println!("Element size: {:?} bytes", mem::size_of_val(&unboxed[0]));
    }
}
