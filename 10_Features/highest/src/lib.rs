#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut arr: Vec<u32> = self.numbers.to_vec();
        arr.sort_by(|a, b| b.cmp(a));
        arr.into_iter().take(3).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        println!("{:?}", n.list());
        println!("{:?}", n.highest());
        println!("{:?}", n.latest());
        println!("{:?}", n.highest_three());
    }
}
