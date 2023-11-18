use std::ops::Range;

use crate::{traits::ContentSequence, Content, Next, simple::simple_block::SimpleTag, SimpleInnerError};

use super::{simple_block::SimpleBlock, SimpleValue, SimpleError};

/// SimpleSection 是用于参数值绑定的 section
#[derive(Clone, Copy)]
pub struct SimpleSection<'section, Contents> 
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
    #[inline]
    fn slice(self, range: Range<usize>) -> Self {
        let rst = Self {
            blocks: &self.blocks[range],
            contents: self.contents,
        };

        rst
    }

    /// 传入实现 Content 的 dto
    #[inline]
    pub fn with<X>(self, content: &X) -> SimpleSection<'section, Next<'section, C, &X>>
    where
        X: Content + ?Sized,
    {
        let rst = SimpleSection {
            blocks: self.blocks,
            contents: self.contents.combine(content),
        };

        rst
    }

    pub fn apply(&self) -> Result<SimpleValue, SimpleError>
    {
        let mut index = 0;
        let mut rst = Err(SimpleInnerError("apply dto error: not found target field".to_owned()).into());
        while let Some(block) = self.blocks.get(index) { 
            index += 1;

            match block.tag {
                SimpleTag::Unescaped => {
                    rst = self.contents.apply_field_unescaped(block.hash, &block.name);
                },
                SimpleTag::Section => {
                    rst = self.contents.apply_field_section(
                        block.hash,
                        &block.name,
                        self.slice(index..index + block.children as usize), 
                    );
                    index += block.children as usize;
                },
            }
        }

        rst
    }
}