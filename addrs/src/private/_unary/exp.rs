use crate::{
    scalar::{Exp, Scalar},
    Expr,
};

use super::{_UOp, _Unary};

impl<T: Scalar + Exp> Exp for Expr<T> {
    #[inline]
    fn exp(self) -> Self {
        let o = self.output().clone().exp();
        let g = o.clone();
        _Unary::create(self, o, g, _UOp::Exp)
    }
}
