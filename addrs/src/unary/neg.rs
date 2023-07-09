use std::{ops::Neg, rc::Rc};

use crate::{scalar::Scalar, Expr};

use super::{_UOp, _Unary};

impl<T: Scalar> Neg for Expr<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        let o = -self.output().clone();
        let i = Rc::new(self._take());
        let is_c = i.is_const();
        let op = _UOp::Neg;
        _Unary { i, o, is_c, op }.into()
    }
}
