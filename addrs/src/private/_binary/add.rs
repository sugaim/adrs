use std::ops::Add;

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Add for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() + rhs.output();
        let gl = T::one();
        let gr = T::one();
        _Binary::create(self, rhs, o, gl, gr, _BOp::Add)
    }
}
