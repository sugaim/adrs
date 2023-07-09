mod exp;
mod log;
mod neg;
mod sqrt;

use std::rc::Rc;

use crate::{
    expr::{Expr, _Expr, _Node},
    scalar::Scalar,
};

#[derive(Debug, Clone)]
enum _UOp {
    Neg,
    Sqrt,
    Exp,
    Log,
}

#[derive(Debug, Clone)]
pub(crate) struct _Unary<T> {
    i: Rc<_Expr<T>>,
    o: T,
    is_c: bool,
    op: _UOp,
}

impl<T> _Unary<T> {
    #[inline]
    pub fn output(&self) -> &T {
        &self.o
    }
    #[inline]
    pub fn is_const(&self) -> bool {
        self.is_c
    }
    #[inline]
    pub fn _ref_expr_for_drop(&mut self) -> Option<&mut _Expr<T>> {
        Rc::get_mut(&mut self.i)
    }
}
impl<T: Scalar> _Unary<T> {
    fn _adj_grad(&self, grad: T) -> T {
        match &self.op {
            _UOp::Neg => -grad,
            _UOp::Sqrt => grad / (T::from(2.0) * self.output()),
            _UOp::Exp => grad * self.output(),
            _UOp::Log => grad / self.i.output(),
        }
    }
    pub fn backward(&self, grad: T) -> (&_Expr<T>, T) {
        (&self.i, self._adj_grad(grad))
    }
}

impl<T> From<_Unary<T>> for Expr<T> {
    fn from(u: _Unary<T>) -> Self {
        let g = u.i.generation() + 1;
        _Expr::Node(g, _Node::Unary(u)).into()
    }
}
