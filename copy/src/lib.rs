pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let res = a.split_whitespace()
        .map(|c| {
            let num: f64 = c.parse().unwrap();
            num.exp().to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");
    (a, res)
}
pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res : Vec<f64> = b.iter().map({|i|
        (*i as f64).abs().ln()
    })
    .collect::<Vec<f64>>();
    (b, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4, 5];
        let c = 0;

        println!("{:?}", nbr_function(c));
        println!("{:?}", vec_function(b));
        println!("{:?}", str_function(a));
    }
}
