use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        if value == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let nums = [
            1000, 900, 500, 400,
            100, 90, 50, 40,
            10, 9, 5, 4, 1,
        ];

        let romans = [
            vec![M], vec![C, M], vec![D], vec![C, D],
            vec![C], vec![X, C], vec![L], vec![X, L],
            vec![X], vec![I, X], vec![V], vec![I, V], vec![I],
        ];

        let mut res = Vec::new();

        for i in 0..nums.len() {
            while value >= nums[i] {
                for d in &romans[i] {
                    res.push(*d);
                }
                value -= nums[i];
            }
        }

        RomanNumber(res)
    }
}

fn dval(d: &RomanDigit) -> u32 {
    match d {
        Nulla => 0,
        I => 1,
        V => 5,
        X => 10,
        L => 50,
        C => 100,
        D => 500,
        M => 1000,
    }
}

fn to_val(n: &RomanNumber) -> u32 {
    let d = &n.0;
    let mut sum = 0;
    for i in 0..d.len() {
        let cur = dval(&d[i]);
        let nxt = d.get(i + 1).map(dval).unwrap_or(0);
        if cur < nxt {
            sum -= cur;
        } else {
            sum += cur;
        }
    }
    sum
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let val = to_val(self) + 1;
        let res = RomanNumber::from(val);
        *self = res.clone();
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut number = RomanNumber::from(15);

        println!("{:?}", number);
        println!("{:?}", number.next());
    }
}