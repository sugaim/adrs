use std::ops::Neg;

use crate::{scalar::Scalar, Expr};

use super::{_UOp, _Unary};

impl<T: Scalar> Neg for Expr<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        let o = -self.output().clone();
        let g = -T::one();
        _Unary::create(self, o, g, _UOp::Neg)
    }
}
