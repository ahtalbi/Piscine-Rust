#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }
        let cr = self.v;
        self.v = if cr % 2 == 0 {
            cr / 2
        } else {
            cr * 3 + 1
        };
        Some(Collatz { v: cr })
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", collatz(0));
        println!("{:?}", collatz(1));
        println!("{:?}", collatz(4));
        println!("{:?}", collatz(5));
        println!("{:?}", collatz(6));
        println!("{:?}", collatz(7));
        println!("{:?}", collatz(12));
    }
}
