use crate::{
    scalar::{Log, Scalar},
    Expr,
};

use super::{_UOp, _Unary};

impl<T: Scalar + Log> Log for Expr<T> {
    #[inline]
    fn log(self) -> Self {
        let o = self.output().clone().log();
        let g = T::one() / self.output();
        _Unary::create(self, o, g, _UOp::Log)
    }
}
