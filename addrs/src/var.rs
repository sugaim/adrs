use std::borrow::Cow;

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub name: Cow<'static, str>,
    pub num: usize,
}

#[derive(Debug, Clone)]
pub struct Var<T> {
    val: T,
    id: Id,
}

impl<T> Var<T> {
    #[inline]
    pub fn id(&self) -> &Id {
        &self.id
    }
    #[inline]
    pub fn val(&self) -> &T {
        &self.val
    }
    #[inline]
    pub fn into_expr(self) -> Expr<T> {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct VarFactory {
    name: Cow<'static, str>,
    cnt: usize,
}

impl VarFactory {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into().into(),
            cnt: 0,
        }
    }
    pub fn gen<T>(&mut self, val: T) -> Var<T> {
        let id = Id {
            name: self.name.clone(),
            num: self.cnt,
        };
        self.cnt += 1;
        Var { val, id }
    }
}
