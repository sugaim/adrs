use crate::{
    scalar::{Scalar, Sqrt},
    Expr,
};

use super::{_UOp, _Unary};

impl<T: Scalar + Sqrt> Sqrt for Expr<T> {
    #[inline]
    fn sqrt(self) -> Self {
        let o = self.output().clone().sqrt();
        let g = T::from(0.5_f64) / &o;
        _Unary::create(self, o, g, _UOp::Sqrt)
    }
}
