// Ramhorns  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Ramhorns. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Ramhorns.  If not, see <http://www.gnu.org/licenses/>

use arrayvec::ArrayVec;
use logos::Logos;
use serde::{Serialize, Deserialize};

use super::{hash_name, Block, TemplateError, Template};
use crate::Partials;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct ParseError;


// r"[^{]+": 跳过非 { 的字符，跳过 \{ 转移的字符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Logos)]
#[logos(
    skip r"[^{]+", 
    skip r"\{",
    extras = Braces,
    error = ParseError,
)]
#[derive(Serialize, Deserialize)]
pub enum Tag {
    /// `{{escaped}}` tag
    #[token("{{")]
    Escaped,

    /// `{{{unescaped}}}` tag
    #[token("{{&")]
    #[token("{{{", |lex| lex.extras = Braces::Three)]
    Unescaped,

    /// `{{#section}}` opening tag (with number of subsequent blocks it contains)
    #[token("{{#")]
    Section,

    /// `{{^inverse}}` section opening tag (with number of subsequent blocks it contains)
    #[token("{{^")]
    Inverse,

    /// `{{/closing}}` section tag
    #[token("{{/")]
    Closing,

    /// `{{!comment}}` tag
    #[token("{{!")]
    Comment,

    // /// `{{>partial}}` tag
    // #[token("{{>")]
    // Partial,

    /// `{{?not_none}}` tag
    #[token("{{?")]
    NotNone,

    /// Tailing html
    Tail,
}

impl From<ParseError> for TemplateError {
    fn from(_: ParseError) -> TemplateError {
        TemplateError::UnclosedTag
    }
}

#[derive(Logos)]
#[logos(
    skip r"[. ]+",
    extras = Braces,
)]
#[derive(Debug)]
enum Closing {
    #[token("}}", |lex| {
        // 如果开始的是 {{ 而结尾的 是 }}} ，则报错
        // Force fail the match if we expected 3 braces
        lex.extras != Braces::Three
    })]
    #[token("}}}")]
    Match,

    #[regex(r"[^. \}]+")]
    Ident,
}

/// Marker of how many braces we expect to match
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Braces {
    Two = 2,
    Three = 3,
}

impl Default for Braces {
    #[inline]
    fn default() -> Self {
        Braces::Two
    }
}

impl Template {
    pub(crate) fn parse(
        &mut self,
        source: &str,
        _partials: &mut impl Partials,
    ) -> Result<usize, TemplateError> {
        let mut last = 0;
        let mut lex = Tag::lexer(source);
        let mut stack = ArrayVec::<usize, 16>::new();

        while let Some(tag) = lex.next() {
            // 起始 token
            let tag = tag?;

            // Grab HTML from before the token
            // TODO: add lex.before() that yields source slice
            // in front of the token:
            //
            // let html = &lex.before()[last..];
            let mut html = &lex.source()[last..lex.span().start];
            self.capacity_hint += html.len();

            // Morphing the lexer to match the closing
            // braces and grab the name
            let mut closing = lex.morph();
            let tail_idx = self.blocks.len();

            // 获取标识符
            let _tok = closing.next();
            if !matches!(Some(Closing::Ident), _tok) {
                return Err(TemplateError::UnclosedTag);
            }
            // 可能是标识符名 或 结束token
            let mut name = closing.slice();

            match tag {
                Tag::Escaped | Tag::Unescaped => {
                    loop {
                        match closing.next() {
                            // 如果起始为 {{ ，在标识符后又找到空格分隔的下一个标识符
                            Some(Ok(Closing::Ident)) => {
                                self.blocks.push(Block::new(html, name, Tag::Section));
                                name = closing.slice();
                                html = "";
                            }
                            // 如果起始为 {{ 则，找到 }}
                            Some(Ok(Closing::Match)) => {
                                self.blocks.push(Block::new(html, name, tag));
                                break;
                            }
                            _ => return Err(TemplateError::UnclosedTag),
                        }
                    }
                    
                    // block 中本次新增元素数
                    let d = self.blocks.len() - tail_idx - 1;

                    for i in 0..d {
                        self.blocks[tail_idx + i].children = (d - i) as u32; //children: 2 1 0 递减
                    }
                }
                Tag::Section | Tag::Inverse => loop {
                    // 如果起始为 {{# ，在标识符后又找到空格分隔的下一个标识符
                    match closing.next() {
                        Some(Ok(Closing::Ident)) => {
                            // 记录上一层的 block 元素数
                            stack.try_push(self.blocks.len())?;
                            self.blocks.push(Block::new(html, name, Tag::Section));
                            name = closing.slice();
                            html = "";
                        }
                        Some(Ok(Closing::Match)) => {
                            // }} 时 记录本层的 block 元素数
                            stack.try_push(self.blocks.len())?;
                            self.blocks.push(Block::new(html, name, tag));
                            break;
                        }
                        _ => return Err(TemplateError::UnclosedTag),
                    }
                },
                Tag::NotNone => loop {
                    match closing.next() {
                        Some(Ok(Closing::Ident)) => {
                            stack.try_push(self.blocks.len())?;
                            self.blocks.push(Block::new(html, name, Tag::NotNone));
                            name = closing.slice();
                            html = "";
                        }
                        Some(Ok(Closing::Match)) => {
                            stack.try_push(self.blocks.len())?;
                            self.blocks.push(Block::new(html, name, tag));
                            break;
                        }
                        _ => return Err(TemplateError::UnclosedTag),
                    }
                }
                Tag::Closing => {
                    self.blocks.push(Block::nameless(html, Tag::Closing));

                    let mut pop_section = |name| {
                        let hash = hash_name(name);

                        let head_idx = stack
                            .pop()
                            .ok_or_else(|| TemplateError::UnopenedSection(name.into()))?;

                        let head = &mut self.blocks[head_idx];
                        head.children = (tail_idx - head_idx) as u32;

                        if head.hash != hash {
                            return Err(TemplateError::UnclosedSection(head.name.clone()));
                        }
                        Ok(())
                    };

                    let mut tmp = vec![name];
                    loop {
                        match closing.next() {
                            Some(Ok(Closing::Ident)) => {
                                tmp.push(closing.slice());
                            }
                            Some(Ok(Closing::Match)) => break,
                            _ => return Err(TemplateError::UnclosedTag),
                        }
                    }

                    let t_len = tmp.len();
                    for i in 0..t_len {
                        pop_section(tmp[t_len - i - 1])?;
                    }

                    // pop_section(name)?;
                    // loop {
                    //     match closing.next() {
                    //         Some(Ok(Closing::Ident)) => {
                    //             pop_section(closing.slice())?;
                    //         }
                    //         Some(Ok(Closing::Match)) => break,
                    //         _ => return Err(Error::UnclosedTag),
                    //     }
                    // }
                }
                // Tag::Partial => {
                //     match closing.next() {
                //         Some(Ok(Closing::Match)) => {}
                //         _ => return Err(TemplateError::UnclosedTag),
                //     }

                //     self.blocks.push(Block::nameless(html, tag));
                //     let partial = partials.get_partial(name)?;
                //     self.blocks.extend_from_slice(&partial.blocks);
                //     self.capacity_hint += partial.capacity_hint;
                // }
                _ => {
                    loop {
                        match closing.next() {
                            Some(Ok(Closing::Ident)) => continue,
                            Some(Ok(Closing::Match)) => break,
                            _ => return Err(TemplateError::UnclosedTag),
                        }
                    }
                    self.blocks.push(Block::nameless(html, tag));
                }
            };

            // Add the number of braces that we were expecting,
            // not the number we got:
            //
            // `{{foo}}}` should not consume the last `}`
            last = closing.span().start + closing.extras as usize;
            lex = closing.morph();
            lex.extras = Braces::Two;
        }

        Ok(last)
    }
}

#[cfg(test)]
mod tests {
    use crate::Template;

    #[test]
    fn test() {
        let s = "
        {{?t1}}
            {{#t1}}
                {{name}}abcd
            {{/t1}}
        {{/t1}}
        ";
        let _tpl = Template::new(s).unwrap();
    }
}