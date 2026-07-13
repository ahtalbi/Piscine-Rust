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
            vec![M],
            vec![C, M],
            vec![D],
            vec![C, D],
            vec![C],
            vec![X, C],
            vec![L],
            vec![X, L],
            vec![X],
            vec![I, X],
            vec![V],
            vec![I, V],
            vec![I],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    println!("{:?}", RomanNumber::from(32));
	    println!("{:?}", RomanNumber::from(9));
	    println!("{:?}", RomanNumber::from(45));
	    println!("{:?}", RomanNumber::from(0));
    }
}
