use std::collections::{HashMap, VecDeque};

use derivative::Derivative;
use num_traits::{One, Zero};

use crate::{binary::_Binary, scalar::Scalar, unary::_Unary, Var};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub(crate) enum _Leaf<T> {
    #[derivative(Debug = "transparent")]
    Var(Var<T>),
    #[derivative(Debug = "transparent")]
    Const(T),
}

impl<T> _Leaf<T> {
    #[inline]
    pub fn val(&self) -> &T {
        match self {
            Self::Var(v) => v.val(),
            Self::Const(c) => c,
        }
    }
    #[inline]
    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }
}

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
    #[inline]
    pub fn is_const(&self) -> bool {
        match self {
            Self::Unary(u) => u.is_const(),
            Self::Binary(b) => {
                let (is_cl, is_cr) = b.is_const_each();
                is_cl && is_cr
            }
        }
    }
}

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
        _Expr::Leaf(_Leaf::Const(T::from(val)))
    }
}

impl<T> _Expr<T> {
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
            Self::_OnlyForDrop => unreachable!(),
            Self::Leaf(l) => l.is_const(),
            Self::Node(_, n) => n.is_const(),
        }
    }
}

impl<T: Scalar> _Expr<T> {
    #[inline]
    pub fn der(&self) -> HashMap<(String, usize), T> {
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

#[derive(Clone, Derivative)]
#[derivative(Debug = "transparent")]
pub struct Expr<T>(_Expr<T>);

impl<T> From<Var<T>> for Expr<T> {
    #[inline]
    fn from(val: Var<T>) -> Self {
        Self(val.into())
    }
}
impl<T: Scalar> From<f64> for Expr<T> {
    #[inline]
    fn from(val: f64) -> Self {
        Expr(val.into())
    }
}
impl<T> From<_Expr<T>> for Expr<T> {
    #[inline]
    fn from(e: _Expr<T>) -> Self {
        Self(e)
    }
}

impl<T> Expr<T> {
    #[inline]
    pub fn output(&self) -> &T {
        self.0.output()
    }
    #[inline]
    pub fn generation(&self) -> usize {
        self.0.generation()
    }
    #[inline]
    pub fn is_const(&self) -> bool {
        self.0.is_const()
    }
    #[inline]
    pub fn as_var(&self) -> Option<&Var<T>> {
        if let _Expr::Leaf(_Leaf::Var(v)) = &self.0 {
            Some(v)
        } else {
            None
        }
    }
    #[inline]
    pub(crate) fn _take(mut self) -> _Expr<T> {
        std::mem::replace(&mut self.0, _Expr::_OnlyForDrop)
    }
}

impl<T: Scalar> Expr<T> {
    #[inline]
    pub fn der(&self) -> HashMap<(String, usize), T> {
        self.0.der()
    }
}

impl<T: PartialEq> PartialEq for Expr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.output() == other.output()
    }
}
impl<T: Eq> Eq for Expr<T> {}

impl<T: PartialOrd> PartialOrd for Expr<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.output().partial_cmp(other.output())
    }
}
impl<T: Ord> Ord for Expr<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.output().cmp(other.output())
    }
}

impl<T: Scalar + PartialOrd> Zero for Expr<T> {
    #[inline]
    fn zero() -> Self {
        Expr(_Expr::Leaf(_Leaf::Const(T::zero())))
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.output().is_zero()
    }
}
impl<T: Scalar + PartialOrd> One for Expr<T> {
    #[inline]
    fn one() -> Self {
        Expr(_Expr::Leaf(_Leaf::Const(T::one())))
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.output().is_one()
    }
}

impl<T> Drop for Expr<T> {
    fn drop(&mut self) {
        if matches!(self.0, _Expr::Leaf(..)) || matches!(self.0, _Expr::_OnlyForDrop) {
            return;
        }

        let mut exprs = VecDeque::new();
        exprs.push_back(std::mem::replace(&mut self.0, _Expr::_OnlyForDrop));
        while let Some(n) = &mut exprs.pop_back() {
            let _Expr::Node(_, n) = n else {
                continue;
            };
            match n {
                _Node::Unary(u) => {
                    let mut i = u._ref_expr_for_drop();
                    if let Some(i) = i.take() {
                        exprs.push_back(std::mem::replace(i, _Expr::_OnlyForDrop));
                    }
                }
                _Node::Binary(b) => {
                    let (mut l, mut r) = b._ref_expr_for_drop();
                    if let Some(l) = l.take() {
                        exprs.push_back(std::mem::replace(l, _Expr::_OnlyForDrop));
                    }
                    if let Some(r) = r.take() {
                        exprs.push_back(std::mem::replace(r, _Expr::_OnlyForDrop));
                    }
                }
            }
        }
    }
}
