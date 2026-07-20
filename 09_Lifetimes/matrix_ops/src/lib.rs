use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use lalgebra_scalar::*;
use matrix::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(arr: [[T; W]; H]) -> Self {
        Wrapper(Matrix(arr))
    }
}

impl<const W: usize, const H: usize, T: Add<Output = T> + Debug + Scalar<Item = T> + Copy> Add
    for Wrapper<W, H, T>
{
    type Output = Wrapper<W, H, T>;
    fn add(self, matrix2: Self) -> Self::Output {
        let mut res = [[T::zero(); W]; H];
        for ele1 in 0..W {
            for ele2 in 0..H {
                res[ele1][ele2] = self.0 .0[ele1][ele2] + matrix2.0 .0[ele1][ele2]
            }
        }
        Wrapper(Matrix(res))
    }
}

impl<const W: usize, const H: usize, T: Sub<Output = T> + Debug + Scalar<Item = T> + Copy> Sub
    for Wrapper<W, H, T>
{
    type Output = Wrapper<W, H, T>;
    fn sub(self, matrix2: Self) -> Self::Output {
        let mut res = [[T::zero(); W]; H];
        for ele1 in 0..W {
            for ele2 in 0..H {
                res[ele1][ele2] = self.0 .0[ele1][ele2] - matrix2.0 .0[ele1][ele2]
            }
        }
        Wrapper(Matrix(res))
    }
}

impl<const S: usize, T: Mul<Output = T>  + Add<Output = T> + Debug + Scalar<Item = T> + Copy> Mul
    for Wrapper<S, S, T>
{
    type Output = Wrapper<S, S, T>;
    fn mul(self, matrix2: Self) -> Self::Output {
        let mut res = [[T::zero(); S]; S];
        res[0][0]=self.0.0[0][0] * matrix2.0.0[0][0] + self.0.0[0][1] * matrix2.0.0[1][0];
        res[0][1]=self.0.0[0][0] * matrix2.0.0[0][1] + self.0.0[0][1] * matrix2.0.0[1][1];
        res[1][0]=self.0.0[1][0] * matrix2.0.0[0][0] + self.0.0[1][1] * matrix2.0.0[1][0];
        res[1][1]=self.0.0[1][0] * matrix2.0.0[0][1] + self.0.0[1][1] * matrix2.0.0[1][1];

        Wrapper(Matrix(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Wrapper::from([[8, 1], [9, 1]]);
        let matrix_2 = Wrapper::from([[1, 1], [1, 1]]);
        println!("{:?}", matrix + matrix_2);
        
        let matrix = Wrapper::from([[1, 3], [2, 5]]);
        let matrix_2 = Wrapper::from([[3, 1], [1, 1]]);
        println!("{:?}", matrix - matrix_2);
        
        let matrix = Wrapper::from([[1, 2], [3, 4]]);
        let matrix_2 = Wrapper::from([[2, 0], [1, 2]]);
        println!("{:?}", matrix * matrix_2);
    }
}
