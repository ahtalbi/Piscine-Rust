pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
}

use std::ops::Add;
impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T: Add<Output = T> + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.beg;
        if self.beg + self.step <= self.end {
            self.beg = self.beg + self.step
        } else {
            return None;
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
		for v in StepIterator::new(0, 100, 10) {
			print!("{},", v);
		}
		println!();

		for v in StepIterator::new(0, 100, 12) {
			print!("{},", v)
		}
		println!();
    }
}
