use std::collections::{HashMap, VecDeque};

use derivative::Derivative;

use crate::{scalar::Scalar, Var};

use super::{_leaf::_Leaf, _node::_Node};

#[derive(Clone, Derivative)]
#[derivative(Debug = "transparent")]
pub(crate) enum _Expr<T> {
    _OnlyForDrop,
    Leaf(_Leaf<T>),
    Node(usize, _Node<T>),
}

impl<T> From<Var<T>> for _Expr<T> {
    #[inline]
    fn from(val: Var<T>) -> Self {
        Self::Leaf(_Leaf::Var(val))
    }
}
impl<T: Scalar> From<f64> for _Expr<T> {
    #[inline]
    fn from(val: f64) -> Self {
        Self::constant(val.into())
    }
}

impl<T> _Expr<T> {
    #[inline]
    pub fn constant(val: T) -> Self {
        Self::Leaf(_Leaf::Const(val))
    }
    #[inline]
    pub fn as_var(&self) -> Option<&Var<T>> {
        if let Self::Leaf(_Leaf::Var(v)) = &self {
            Some(v)
        } else {
            None
        }
    }
    #[inline]
    pub fn output(&self) -> &T {
        match &self {
            Self::_OnlyForDrop => unreachable!(),
            Self::Leaf(l) => l.val(),
            Self::Node(_, n) => n.output(),
        }
    }
    #[inline]
    pub fn generation(&self) -> usize {
        match &self {
            Self::_OnlyForDrop => 0,
            Self::Leaf(_) => 0,
            Self::Node(g, _) => *g,
        }
    }
    #[inline]
    pub fn is_const(&self) -> bool {
        match &self {
            Self::_OnlyForDrop => true,
            Self::Leaf(l) => l.is_const(),
            Self::Node(_, n) => n.is_const(),
        }
    }
}

impl<T: Scalar> _Expr<T> {
    pub fn grads(&self) -> HashMap<(String, usize), T> {
        let mut res = HashMap::new();
        let mut nodes = VecDeque::new();
        nodes.push_back((self, T::one()));
        while let Some((node, grad)) = nodes.pop_back() {
            match &node {
                _Expr::_OnlyForDrop => unreachable!(),
                _Expr::Leaf(leaf) => match leaf {
                    _Leaf::Var(v) => {
                        let (name, id) = v.ident();
                        res.entry((name.to_owned(), id))
                            .and_modify(|x| *x += &grad)
                            .or_insert(grad);
                    }
                    _Leaf::Const(_) => {}
                },
                _Expr::Node(_, n) => match n {
                    _Node::Unary(u) => {
                        if !u.is_const() {
                            nodes.push_back(u.backward(grad));
                        }
                    }
                    _Node::Binary(b) => match b.is_const_each() {
                        (true, true) => {}
                        (true, false) => {
                            nodes.push_back(b.backward_r(grad));
                        }
                        (false, true) => {
                            nodes.push_back(b.backward_l(grad));
                        }
                        (false, false) => {
                            nodes.push_back(b.backward_l(grad.clone()));
                            nodes.push_back(b.backward_r(grad));
                        }
                    },
                },
            }
        }
        res
    }
}
