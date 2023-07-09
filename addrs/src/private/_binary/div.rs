use std::ops::Div;

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Div for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() / rhs.output();
        let gl = T::one() / rhs.output();
        let gr = -o.clone() / rhs.output();
        _Binary::create(self, rhs, o, gl, gr, _BOp::Div)
    }
}
