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
        matches!(self, Self::Leaf(_Leaf::Const(_)))
    }
}

#[allow(dead_code)]
impl<T: Scalar> _Expr<T> {
    #[inline]
    pub fn grads(&self, seed: T) -> HashMap<(String, usize), T> {
        self.grads_v1(seed)
    }

    fn grads_v1(&self, seed: T) -> HashMap<(String, usize), T> {
        let mut res = HashMap::new();
        let mut grads = VecDeque::new();
        grads.push_back((self, seed));
        while let Some((node, grad)) = grads.pop_back() {
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
                _Expr::Node(_, n) => n.push_grads(&mut grads, grad),
            }
        }
        res
    }
}
