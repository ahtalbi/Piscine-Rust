use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(mut self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {return None;}
        for (i, v) in self.0.iter_mut().enumerate() {
            *v = *v + rhs.0[i];
        }
        Some(self)
    }
}

impl<T: Scalar<Item = T>> Vector<T> {
	pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let mut res = T::zero();
        for (i, v) in self.0.iter().enumerate() {
            res = res + (*v * rhs.0[i]);
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    println!("{:?}", Vector(vec![1, 3, -5]).dot(Vector(vec![4, -2, -1])));
	    println!("{:?}", Vector(vec![1, 3, -5]) + Vector(vec![4, -2, -1]));
    }
}

pub trait Scalar:
	Sized
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
	fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u64 {
    type Item = u64;
	fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i32 {
    type Item = i32;
	fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i64 {
    type Item = i64;
	fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f32 {
    type Item = f32;
	fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}
impl Scalar for f64 {
    type Item = f64;
	fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}