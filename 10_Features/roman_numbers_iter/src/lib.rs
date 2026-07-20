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

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 || n >= 50000 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let roman: Vec<(u32, [RomanDigit; 2])> = vec![
            (1000, [M, Nulla]),
            (900, [C, M]),
            (500, [D, Nulla]),
            (400, [C, D]),
            (100, [C, Nulla]),
            (90, [X, C]),
            (50, [L, Nulla]),
            (40, [X, L]),
            (10, [X, Nulla]),
            (9, [I, X]),
            (5, [V, Nulla]),
            (4, [I, V]),
            (1, [I, Nulla])
        ];

        let mut result = Vec::new();

        for (value, digits) in roman.iter() {
            while n >= *value {
                n -= *value;
                result.push(digits[0]);
                if digits[1] != Nulla {
                    result.push(digits[1]);
                }
            }
        }

        RomanNumber(result)
    }
}
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let a = roman_digits_to_number(&self.0) + 1;
        if a >= 50000 {
            return None;
        }
        let next = RomanNumber::from(a);
        Some(next)
    }
}

fn roman_digits_to_number(digits: &[RomanDigit]) -> u32 {
    fn value(d: RomanDigit) -> u32 {
        match d {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
            _ => 0,
        }
    }

    let mut total = 0;
    let mut i = 0;

    while i < digits.len() {
        let current = value(digits[i]);
        let next = if i + 1 < digits.len() { value(digits[i + 1]) } else { 0 };

        if current < next {
            total += next - current;
            i += 2;
        } else {
            total += current;
            i += 1;
        }
    }

    total
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