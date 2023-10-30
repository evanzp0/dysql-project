// Ramhorns  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Ramhorns. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Ramhorns.  If not, see <http://www.gnu.org/licenses/>

use super::{Block, Tag};
use crate::encoding::Encoder;
use crate::Content;
use crate::traits::{Combine, ContentSequence};
use std::ops::Range;

/// A section of a `Template` that can be rendered individually, usually delimited by
/// `{{#section}} ... {{/section}}` tags.
#[derive(Clone, Copy)]
pub struct Section<'section, Contents: ContentSequence> {
    blocks: &'section [Block<'section>],
    contents: Contents,
}

/// Necessary so that the warning of very complex type created when compiling
/// with `cargo clippy` doesn't propagate to downstream crates
type Next<C, X> = (<C as Combine>::I, <C as Combine>::J, <C as Combine>::K, X);

impl<'section> Section<'section, ()> {
    #[inline]
    pub(crate) fn new(blocks: &'section [Block<'section>]) -> Self {
        let rst = Self {
            blocks,
            contents: (),
        };

        rst
    }
}

impl<'section, C> Section<'section, C>
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

    /// Attach a `Content` to this section. This will keep track of a stack up to
    /// 4 `Content`s deep, cycling on overflow.
    #[inline]
    pub fn with<X>(self, content: &X) -> Section<'section, Next<C, &X>>
    where
        X: Content + ?Sized,
    {
        let rst = Section {
            blocks: self.blocks,
            contents: self.contents.combine(content),
        };

        rst
    }

    /// The section without the last `Content` in the stack
    #[inline]
    pub fn without_last(self) -> Section<'section, C::Previous>
    {
        let rst = Section {
            blocks: self.blocks,
            contents: self.contents.crawl_back(),
        };

        rst
    }

    /// Render this section once to the provided `Encoder`.
    pub fn render<E, IC: Content>(&self, encoder: &mut E, content: Option<&IC>) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let mut index = 0;

        while let Some(block) = self.blocks.get(index) { // 消耗本次 render 所需的一层 block
            index += 1;
            encoder.write_unescaped(block.html)?;

            match block.tag {
                Tag::Escaped => {
                    if block.name == "$value" {
                        if let Some(content) = content {
                            content.render_escaped(encoder)?; 
                        }
                    } else {
                        self.contents.render_field_escaped(block.hash, block.name, encoder)?;
                    }
                }
                Tag::Unescaped => {
                    if block.name == "$value" {
                        if let Some(content) = content {
                            content.render_unescaped(encoder)?; 
                        }
                    } else {
                        self.contents.render_field_unescaped(block.hash, block.name, encoder)?;
                    }
                    
                }
                Tag::Section => {
                    self.contents.render_field_section(
                        block.hash, // block0.hash，block0.child = 2
                        block.name,  // block0.name
                        self.slice(index..index + block.children as usize), // 消去本次 render_field_section 后剩下的 子blocks[1，2]， block3 不是 block0 的子 block
                        encoder,
                    )?;
                    index += block.children as usize;
                }
                Tag::Inverse => {
                    self.contents.render_field_inverse(
                        block.hash,
                        block.name,
                        self.slice(index..index + block.children as usize),
                        encoder,
                    )?;
                    index += block.children as usize;
                }
                Tag::NotNone => {
                    let rst = self.contents.render_field_notnone_section(
                        block.hash,
                        block.name,
                        self.slice(index..index + block.children as usize),
                        encoder,
                    )?;

                    if !rst {
                        index += block.children as usize;
                    }
                    // index += block.children as usize;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
