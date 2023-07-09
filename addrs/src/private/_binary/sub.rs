use std::ops::Sub;

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Sub for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() - rhs.output();
        let gl = T::one();
        let gr = -T::one();
        _Binary::create(self, rhs, o, gl, gr, _BOp::Sub)
    }
}
