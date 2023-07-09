use std::rc::Rc;

use crate::{scalar::Sqrt, Expr};

use super::{_UOp, _Unary};

impl<T: Sqrt + Clone> Sqrt for Expr<T> {
    #[inline]
    fn sqrt(self) -> Self {
        let o = self.output().clone().sqrt();
        let i = Rc::new(self._take());
        let is_c = i.is_const();
        let op = _UOp::Sqrt;
        _Unary { i, o, is_c, op }.into()
    }
}
