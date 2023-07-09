use std::rc::Rc;

use crate::{scalar::Log, Expr};

use super::{_UOp, _Unary};

impl<T: Log + Clone> Log for Expr<T> {
    #[inline]
    fn log(self) -> Self {
        let o = self.output().clone().log();
        let i = Rc::new(self._take());
        let is_c = i.is_const();
        let op = _UOp::Log;
        _Unary { i, o, is_c, op }.into()
    }
}
