use std::marker::PhantomData;

use crate::{traits::ContentSequence, Content, Next};

use super::SimpleBlock;

/// SimpleSection 是用于参数值绑定的 section
#[derive(Clone, Copy)]
pub struct SimpleSection<'section, Contents, F, V, Q> 
where 
    Contents: ContentSequence,
    F: FnOnce(V, Q) -> Q,
{
    blocks: &'section [SimpleBlock],
    contents: Contents,
    binder: F,
    _v: PhantomData<V>,
    _q: PhantomData<Q>,
}

impl<'section, F, V, Q> SimpleSection<'section, (), F, V, Q> 
where
    F: FnOnce(V, Q) -> Q,
{
    #[inline]
    pub(crate) fn new(blocks: &'section [SimpleBlock], binder: F) -> Self
    {

        let rst = Self {
            blocks,
            contents: (),
            binder,
            _v: PhantomData,
            _q: PhantomData,
        };

        rst
    }
}

impl<'section, C, F, V, Q> SimpleSection<'section, C, F, V, Q> 
where
    C: ContentSequence,
    F: FnOnce(V, Q) -> Q,
{
    /// 传入实现 Content 的 dto
    #[inline]
    pub(crate) fn with<X>(self, content: &X) -> SimpleSection<'section, Next<C, &X>, F, V, Q>
    where
        X: Content + ?Sized,
    {
        let rst = SimpleSection {
            blocks: self.blocks,
            contents: self.contents.combine(content),
            binder: self.binder,
            _v: PhantomData,
            _q: PhantomData,
        };

        rst
    }

    pub(crate) fn apply(&self) {
        
    }
}