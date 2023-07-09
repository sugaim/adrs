use std::collections::VecDeque;

use derivative::Derivative;

use crate::scalar::Scalar;

use super::{_Expr, _binary::_Binary, _unary::_Unary};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub(crate) enum _Node<T> {
    #[derivative(Debug = "transparent")]
    Unary(_Unary<T>),
    #[derivative(Debug = "transparent")]
    Binary(_Binary<T>),
}

impl<T> _Node<T> {
    #[inline]
    pub fn output(&self) -> &T {
        match self {
            Self::Unary(u) => u.output(),
            Self::Binary(b) => b.output(),
        }
    }
    pub fn _take_expr_to_back_for_drop(&mut self, buf: &mut VecDeque<_Expr<T>>) {
        match self {
            _Node::Unary(u) => {
                let mut i = u._ref_expr_for_drop();
                if let Some(i) = i.take() {
                    buf.push_back(std::mem::replace(i, _Expr::_OnlyForDrop));
                }
            }
            _Node::Binary(b) => {
                let (mut l, mut r) = b._ref_expr_for_drop();
                if let Some(l) = l.take() {
                    buf.push_back(std::mem::replace(l, _Expr::_OnlyForDrop));
                }
                if let Some(r) = r.take() {
                    buf.push_back(std::mem::replace(r, _Expr::_OnlyForDrop));
                }
            }
        }
    }
}

impl<T: Scalar> _Node<T> {
    #[inline]
    pub fn push_grads<'a>(&'a self, grads: &mut VecDeque<(&'a _Expr<T>, T)>, grad: T) {
        match self {
            Self::Unary(u) => u.push_grads(grads, grad),
            Self::Binary(b) => b.push_grads(grads, grad),
        }
    }
}
