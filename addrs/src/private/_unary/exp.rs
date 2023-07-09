use std::rc::Rc;

use crate::{scalar::Exp, Expr};

use super::{_UOp, _Unary};

impl<T: Exp + Clone> Exp for Expr<T> {
    #[inline]
    fn exp(self) -> Self {
        let o = self.output().clone().exp();
        let i = Rc::new(self._take());
        let is_c = i.is_const();
        let op = _UOp::Exp;
        _Unary { i, o, is_c, op }.into()
    }
}
