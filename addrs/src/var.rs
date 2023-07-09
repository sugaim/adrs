use std::borrow::Cow;

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub group: usize,
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
pub struct VarGroup {
    id: usize,
    name: Cow<'static, str>,
    cnt: usize,
}

impl VarGroup {
    pub fn new(name: impl Into<String>) -> Self {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let name = name.into().into();
        let cnt = 0;
        Self { id, name, cnt }
    }
    pub fn gen<T>(&mut self, val: T) -> Var<T> {
        let id = Id {
            group: self.id,
            name: self.name.clone(),
            num: self.cnt,
        };
        self.cnt += 1;
        Var { val, id }
    }
}
