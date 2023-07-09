use derivative::Derivative;

use crate::Var;

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
