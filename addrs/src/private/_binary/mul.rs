use std::ops::Mul;

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Mul for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() * rhs.output();
        let gl = rhs.output().clone();
        let gr = self.output().clone();
        _Binary::create(self, rhs, o, gl, gr, _BOp::Mul)
    }
}
