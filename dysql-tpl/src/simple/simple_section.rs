use crate::{traits::ContentSequence, Content, Next};

use super::{simple_block::SimpleBlock, SimpleValue};

/// SimpleSection 是用于参数值绑定的 section
#[derive(Clone, Copy)]
pub(crate) struct SimpleSection<'section, Contents> 
where 
    Contents: ContentSequence,
{
    blocks: &'section [SimpleBlock],
    contents: Contents,
}

impl<'section> SimpleSection<'section, ()> 
{
    #[inline]
    pub(crate) fn new(blocks: &'section [SimpleBlock]) -> Self
    {

        let rst = Self {
            blocks,
            contents: (),
        };

        rst
    }
}

impl<'section, C> SimpleSection<'section, C> 
where
    C: ContentSequence,
{
    /// 传入实现 Content 的 dto
    #[inline]
    pub(crate) fn with<X>(self, content: &X) -> SimpleSection<'section, Next<C, &X>>
    where
        X: Content + ?Sized,
    {
        let rst = SimpleSection {
            blocks: self.blocks,
            contents: self.contents.combine(content),
        };

        rst
    }

    pub(crate) fn apply(&self) -> SimpleValue {
        todo!()
    }
}