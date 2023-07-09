mod exp;
mod log;
mod neg;
mod sqrt;

use std::{collections::VecDeque, rc::Rc};

use crate::{expr::Expr, scalar::Scalar};

use super::{_expr::_Expr, _node::_Node};

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
    g: T,
    #[allow(dead_code)]
    op: _UOp,
}

impl<T> _Unary<T> {
    #[inline]
    fn create(i: Expr<T>, o: T, g: T, op: _UOp) -> Expr<T> {
        if i._is_const() {
            return Expr::constant(o);
        }
        let i = Rc::new(i._take());
        let gen = i.generation() + 1;
        let u = _Unary { i, o, g, op };
        _Expr::Node(gen, _Node::Unary(u)).into()
    }

    #[inline]
    pub fn output(&self) -> &T {
        &self.o
    }
    #[inline]
    pub fn _ref_expr_for_drop(&mut self) -> Option<&mut _Expr<T>> {
        Rc::get_mut(&mut self.i)
    }
}
impl<T: Scalar> _Unary<T> {
    pub fn push_grads<'a>(&'a self, grads: &mut VecDeque<(&'a _Expr<T>, T)>, grad: T) {
        if !self.i.is_const() {
            grads.push_back((&self.i, grad * &self.g));
        }
    }
}
