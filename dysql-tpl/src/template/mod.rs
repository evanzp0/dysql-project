// Ramhorns  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Ramhorns. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Ramhorns.  If not, see <http://www.gnu.org/licenses/>

use std::fs::File;
use std::io;
use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::encoding::EscapingIOEncoder;
use crate::{Partials, hash_name};
use crate::{Content, TemplateError};

mod parse;
mod section;

pub use section::Section;
pub use parse::Tag;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
/// A preprocessed form of the plain text template, ready to be rendered
/// with data contained in types implementing the `Content` trait.
pub struct Template {
    /// Parsed blocks!
    blocks: Vec<Block>,

    /// Total byte length of all the blocks, used to estimate preallocations.
    capacity_hint: usize,

    /// Source from which this template was parsed.
    source: String,
}

impl Template {
    /// Create a new `Template` out of the source.
    ///
    /// + If `source` is a `&str`, this `Template` will borrow it with appropriate lifetime.
    /// + If `source` is a `String`, this `Template` will take it's ownership (The `'tpl` lifetime will be `'static`).
    pub fn new(source: &str) -> Result<Self, TemplateError>
    {
        Template::load(source, &mut NoPartials)
    }

    pub(crate) fn load(source: &str, partials: &mut impl Partials) -> Result<Self, TemplateError>
    {
        let source = source.to_owned();

        // This is allows `Block`s inside this `Template` to be references of the `source` field.
        // This is safe as long as the `source` field is never mutated or dropped.
        let unsafe_source: &str = unsafe { &*(&*source as *const str) };

        let mut tpl = Template {
            blocks: Vec::with_capacity(16),
            capacity_hint: 0,
            source,
        };
        let last = tpl.parse(unsafe_source, partials)?;
        let tail = &unsafe_source[last..].trim_end();
        tpl.blocks.push(Block::nameless(tail, Tag::Tail));
        tpl.capacity_hint += tail.len();

        Ok(tpl)
    }

    /// Estimate how big of a buffer should be allocated to render this `Template`.
    pub fn capacity_hint(&self) -> usize {
        self.capacity_hint
    }

    /// Render this `Template` with a given `Content` to a `String`.
    pub fn render<C>(&self, content: &C) -> String 
    where
        for<'section> C: Content
    {
        let mut capacity = content.capacity_hint(self);

        // Add extra 25% extra capacity for HTML escapes and an odd double variable use.
        capacity += capacity / 4;

        let mut buf = String::with_capacity(capacity);

        // Ignore the result, cannot fail
        let rst = Section::new(&self.blocks)
            .with(content);
        
        let _ = rst.render(&mut buf, Option::<&()>::None);

        buf
    }

    /// Render this `Template` with a given `Content` to a writer.
    pub fn render_to_writer<W, C>(&self, writer: &mut W, content: &C) -> io::Result<()>
    where
        W: io::Write,
        for<'section> C: Content,
    {
        let mut encoder = EscapingIOEncoder::new(writer);
        Section::new(&self.blocks)
            .with(content)
            .render(&mut encoder, Option::<&()>::None)
    }

    /// Render this `Template` with a given `Content` to a file.
    pub fn render_to_file<P, C>(&self, path: P, content: &C) -> io::Result<()>
    where
        P: AsRef<Path>,
        for<'section> C: Content,
    {
        use io::BufWriter;

        let writer = BufWriter::new(File::create(path)?);
        let mut encoder = EscapingIOEncoder::new(writer);

        Section::new(&self.blocks)
            .with(content)
            .render(&mut encoder, Option::<&()>::None)
    }

    /// Get a reference to a source this `Template` was created from.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Serialize template to Vec<u8>
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("error: serialize template failed")
    }

    /// Deserialize template from Vec<u8>
    pub fn deserialize(bin_content: &[u8]) -> Self {
        bincode::deserialize(bin_content).expect("error: deserialize template failed")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Block {
    html: String,
    name: String,
    hash: u64,
    tag: Tag,
    children: u32,
}

impl Block {
    #[inline]
    fn new(html: &str, name: &str, tag: Tag) -> Self {
        Block {
            html: html.to_owned(),
            name: name.to_owned(),
            hash: hash_name(name),
            tag,
            children: 0,
        }
    }

    // Skips hashing; can be used when tag is Partial, Comment or Tail
    #[inline]
    fn nameless(html: &str, tag: Tag) -> Self {
        Block {
            html: html.to_owned(),
            name: "".to_owned(),
            hash: 0,
            tag,
            children: 0,
        }
    }
}

struct NoPartials;

impl Partials for NoPartials {
    fn get_partial(&mut self, _name: &str) -> Result<&Template, TemplateError> {
        Err(TemplateError::PartialsDisabled)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    impl Block {
        fn children(self, children: u32) -> Self {
            Block { children, ..self }
        }
    }

    #[test]
    fn template_from_string_is_static() {
        let tpl: Template = Template::new(&String::from("Ramhorns")).unwrap();

        assert_eq!(tpl.source(), "Ramhorns");
    }

    #[test]
    fn block_hashes_correctly() {
        assert_eq!(
            Block::new("", "test", Tag::Escaped),
            Block {
                html: "".to_owned(),
                name: "test".to_owned(),
                hash: 2271575940368597870,
                tag: Tag::Escaped,
                children: 0,
            }
        );
    }

    #[test]
    fn constructs_blocks_correctly() {
        let source = "<title>{{title}}</title><h1>{{title}}</h1><div>{{{body}}}</div>";
        let tpl = Template::new(source).unwrap();

        assert_eq!(
            &tpl.blocks,
            &[
                Block::new("<title>", "title", Tag::Escaped),
                Block::new("</title><h1>", "title", Tag::Escaped),
                Block::new("</h1><div>", "body", Tag::Unescaped),
                Block::nameless("</div>", Tag::Tail),
            ]
        );
    }

    #[test]
    fn constructs_nested_sections_correctly() {
        let source = "<body><h1>{{title}}</h1>{{#posts}}<article>{{name}}</article>{{/posts}}{{^posts}}<p>Nothing here :(</p>{{/posts}}</body>";
        let tpl = Template::new(source).unwrap();

        assert_eq!(
            &tpl.blocks,
            &[
                Block::new("<body><h1>", "title", Tag::Escaped),
                Block::new("</h1>", "posts", Tag::Section).children(2),
                Block::new("<article>", "name", Tag::Escaped),
                Block::nameless("</article>", Tag::Closing),
                Block::new("", "posts", Tag::Inverse).children(1),
                Block::nameless("<p>Nothing here :(</p>", Tag::Closing),
                Block::nameless("</body>", Tag::Tail),
            ]
        );
    }

    #[test]
    fn constructs_nested_sections_with_dot_correctly() {
        let source = "<body><h1>{{site title}}</h1>{{^archive posts}}<article>{{name}}</article>{{/archive posts}}</body>";
        let tpl = Template::new(source).unwrap();

        assert_eq!(
            &tpl.blocks,
            &[
                Block::new("<body><h1>", "site", Tag::Section).children(1),
                Block::new("", "title", Tag::Escaped),
                Block::new("</h1>", "archive", Tag::Section).children(3),
                Block::new("", "posts", Tag::Inverse).children(2),
                Block::new("<article>", "name", Tag::Escaped),
                Block::nameless("</article>", Tag::Closing),
                Block::nameless("</body>", Tag::Tail),
            ]
        );
    }
}
