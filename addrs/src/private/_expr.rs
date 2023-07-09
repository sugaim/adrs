use std::collections::{BTreeMap, VecDeque};

use derivative::Derivative;

use crate::{scalar::Scalar, var::Id, Var};

use super::{_leaf::_Leaf, _node::_Node};

#[derive(Clone, Derivative)]
#[derivative(Debug = "transparent")]
pub(crate) enum _Expr<T> {
    _OnlyForDrop,
    Leaf(_Leaf<T>),
    Node(usize, _Node<T>),
    Compressed { g: usize, o: T, gs: BTreeMap<Id, T> },
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
            Self::Compressed { o, .. } => o,
        }
    }
    #[inline]
    pub fn generation(&self) -> usize {
        match &self {
            Self::_OnlyForDrop => 0,
            Self::Leaf(_) => 0,
            Self::Node(g, _) => *g,
            Self::Compressed { g, .. } => *g,
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
    pub fn grads(&self, seed: T) -> BTreeMap<Id, T> {
        self.grads_v1(seed)
    }

    fn grads_v1(&self, seed: T) -> BTreeMap<Id, T> {
        if let Self::Compressed { gs, .. } = self {
            let mut res = gs.clone();
            for (_, g) in res.iter_mut() {
                *g *= &seed;
            }
            return res;
        }
        let mut res = BTreeMap::new();
        let mut grads = VecDeque::new();
        grads.push_back((self, seed));
        while let Some((node, grad)) = grads.pop_back() {
            match &node {
                _Expr::_OnlyForDrop => unreachable!(),
                _Expr::Leaf(leaf) => match leaf {
                    _Leaf::Var(v) => {
                        res.entry(v.id().clone())
                            .and_modify(|x| *x += &grad)
                            .or_insert(grad);
                    }
                    _Leaf::Const(_) => {}
                },
                _Expr::Node(_, n) => n.push_grads(&mut grads, grad),
                _Expr::Compressed { gs, .. } => {
                    for (id, g) in gs {
                        let g = grad.clone() * g;
                        res.entry(id.clone()).and_modify(|x| *x += &g).or_insert(g);
                    }
                }
            }
        }
        res
    }
}
