// Ramhorns  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Ramhorns. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Ramhorns.  If not, see <http://www.gnu.org/licenses/>

//! Utilities dealing with writing the bits of a template or data to the output and
//! escaping special HTML characters.

use std::io;
use std::fmt;

#[cfg(feature = "pulldown-cmark")]
use pulldown_cmark::{html, Event, Parser};

use crate::SimpleError;
use crate::SimpleInnerError;

/// A trait that wraps around either a `String` or `std::io::Write`, providing UTF-8 safe
/// writing boundaries and special HTML character escaping.
pub trait Encoder {
    /// Error type for this encoder
    type Error;

    /// Write a `&str` to this `Encoder` in plain mode.
    fn write_unescaped(&mut self, part: &str) -> Result<(), Self::Error>;

    /// Write a `&str` to this `Encoder`, escaping special HTML characters.
    fn write_escaped(&mut self, part: &str) -> Result<(), Self::Error>;

    #[cfg(feature = "pulldown-cmark")]
    /// Write HTML from an `Iterator` of `pulldown_cmark` `Event`s.
    fn write_html<'b, I: Iterator<Item = Event<'b>>>(&mut self, iter: I) -> Result<(), Self::Error>;

    /// Write a `Display` implementor to this `Encoder` in plain mode.
    fn format_unescaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error>;

    /// Write a `Display` implementor to this `Encoder`, escaping special HTML characters.
    fn format_escaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error>;
}

/// Local helper for escaping stuff into strings.
struct EscapingStringEncoder<'a>(&'a mut String);

impl<'a> EscapingStringEncoder<'a> {
    /// Write with escaping special HTML characters. Since we are dealing
    /// with a String, we don't need to return a `Result`.
    fn write_escaped(&mut self, part: &str) {
        let mut start = 0;

        for (idx, byte) in part.bytes().enumerate() {
            let replace = match byte {
                b'<' => "&lt;",
                b'>' => "&gt;",
                b'&' => "&amp;",
                b'"' => "&quot;",
                _ => continue,
            };

            self.0.push_str(&part[start..idx]);
            self.0.push_str(replace);

            start = idx + 1;
        }

        self.0.push_str(&part[start..]);
    }
}

/// Provide a `fmt::Write` interface, so we can use `write!` macro.
impl<'a> fmt::Write for EscapingStringEncoder<'a> {
    #[inline]
    fn write_str(&mut self, part: &str) -> fmt::Result {
        self.write_escaped(part);

        Ok(())
    }
}

/// Encoder wrapper around io::Write. We can't implement `Encoder` on a generic here,
/// because we're implementing it directly for `String`.
pub(crate) struct EscapingIOEncoder<W: io::Write> {
    inner: W,
}

impl<W: io::Write> EscapingIOEncoder<W> {
    #[inline]
    pub fn new(inner: W) -> Self {
        Self {
            inner
        }
    }

    /// Same as `EscapingStringEncoder`, but dealing with byte arrays and writing to
    /// the inner `io::Write`.
    fn write_escaped_bytes(&mut self, part: &[u8]) -> io::Result<()> {
        let mut start = 0;

        for (idx, byte) in part.iter().enumerate() {
            let replace: &[u8] = match *byte {
                b'<' => b"&lt;",
                b'>' => b"&gt;",
                b'&' => b"&amp;",
                b'"' => b"&quot;",
                _ => continue,
            };

            self.inner.write_all(&part[start..idx])?;
            self.inner.write_all(replace)?;

            start = idx + 1;
        }

        self.inner.write_all(&part[start..])
    }
}

// Additionally we implement `io::Write` for it directly. This allows us to use
// the `write!` macro for formatting without allocations.
impl<W: io::Write> io::Write for EscapingIOEncoder<W> {
    #[inline]
    fn write(&mut self, part: &[u8]) -> io::Result<usize> {
        self.write_escaped_bytes(part).map(|()| part.len())
    }

    #[inline]
    fn write_all(&mut self, part: &[u8]) -> io::Result<()> {
        self.write_escaped_bytes(part)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<W: io::Write> Encoder for EscapingIOEncoder<W> {
    type Error = io::Error;

    #[inline]
    fn write_unescaped(&mut self, part: &str) -> io::Result<()> {
        self.inner.write_all(part.as_bytes())
    }

    #[inline]
    fn write_escaped(&mut self, part: &str) -> io::Result<()> {
        self.write_escaped_bytes(part.as_bytes())
    }

    #[cfg(feature = "pulldown-cmark")]
    #[inline]
    fn write_html<'b, I: Iterator<Item = Event<'b>>>(&mut self, iter: I) -> io::Result<()> {
        html::write_html(&mut self.inner, iter)
    }

    #[inline]
    fn format_unescaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        write!(self.inner, "{}", display)
    }

    #[inline]
    fn format_escaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        use io::Write;

        write!(self, "{}", display)
    }
}

/// Error type for `String`, impossible to instantiate.
/// Rust optimizes `Result<(), NeverError>` to 0-size.
pub enum NeverError {}

impl Encoder for String {
    // Change this to `!` once stabilized.
    type Error = NeverError;

    #[inline]
    fn write_unescaped(&mut self, part: &str) -> Result<(), Self::Error> {
        self.push_str(part);

        Ok(())
    }

    #[inline]
    fn write_escaped(&mut self, part: &str) -> Result<(), Self::Error> {
        EscapingStringEncoder(self).write_escaped(part);

        Ok(())
    }

    #[cfg(feature = "pulldown-cmark")]
    #[inline]
    fn write_html<'b, I: Iterator<Item = Event<'b>>>(&mut self, iter: I) -> Result<(), Self::Error> {
        html::push_html(self, iter);

        Ok(())
    }

    #[inline]
    fn format_unescaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        use std::fmt::Write;

        // Never fails for a string
        let _ = write!(self, "{}", display);

        Ok(())
    }

    #[inline]
    fn format_escaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        use std::fmt::Write;

        // Never fails for a string
        let _ = write!(EscapingStringEncoder(self), "{}", display);

        Ok(())
    }
}

#[cfg(feature = "pulldown-cmark")]
/// Parse and encode the markdown using pulldown_cmark
pub fn encode_cmark<E: Encoder>(source: &str, encoder: &mut E) -> Result<(), E::Error> {
    let parser = Parser::new(source);

    encoder.write_html(parser)
}

enum Token<'a> {
    // (含控制符的 len, token str)
    DEL(usize, &'a str), 
    Normal(usize, &'a str),
}

const BLANKET_CHARS: [u8; 3] = [b' ', b'\n', b'\t'];

pub(crate) struct SqlEncoder
{
    pub inner: String,
    pub trim_token: Option<String>,
}

impl SqlEncoder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: String::with_capacity(capacity),
            trim_token: None,
        }
    }

    pub fn trim(mut self) -> String {
        if self.inner.len() > 0 {
            self.inner.pop();
        }
        self.inner
    }

    fn trim_sql(&mut self, sql: &str) -> Result<(), SimpleError>
    {
        let sql_buf = &mut self.inner;
        let mut end: usize = 0; // 当前位置
        let sql_len: usize = sql.len();
        // let mut trim_token: Option<&str> = None;
    
        while end < sql_len { 
            if let Some(idx) = skip_blank(&sql[end..sql_len]) {
                end += idx;
            } else {
                break;
            }
            
            let sql_token = get_token(&sql[end..sql_len])?;
            let token_len = match sql_token {
                // 如果是 DEL 控制符，则记录需要 trim 的 token，在下一次写入 sql_buf 时过滤字符串
                Token::DEL(len, token) => {
                    self.trim_token = Some(token.to_owned());
                    len
                }
                // 需要输出的 sql token 如果开始位置有需要 DEL 的 token，则跳过此 token 写入 sql_buf,
                // 写入后重置 trim_token 为 None.
                Token::Normal(len, token) => {
                    if let Some(tm_token) = &self.trim_token {
                        let trim_len = tm_token.len();
                        if trim_len <= token.len() && tm_token == &token[0..trim_len] {
                            if trim_len < token.len() {
                                sql_buf.push_str(&token[trim_len..]);
                                sql_buf.push_str(" ");
                            } else {
                                sql_buf.push_str(&token[trim_len..]);
                            }
                        } else {
                            sql_buf.push_str(token);
                            sql_buf.push_str(" ");
                        }
                    } else {
                        sql_buf.push_str(token);
                        sql_buf.push_str(" ");
                    }
                    self.trim_token = None;
    
                    len
                }
            };
    
            end += token_len;
        }

        Ok(())
    }
}

impl  Encoder for SqlEncoder  {
    // Change this to `!` once stabilized.
    type Error = SimpleError;

    #[inline]
    fn write_unescaped(&mut self, part: &str) -> Result<(), Self::Error> {
        
        self.trim_sql(part)?;

        // println!("unescaped | bf:{}/ af:{}/", part, self.inner);

        Ok(())
    }

    #[inline]
    fn write_escaped(&mut self, part: &str) -> Result<(), Self::Error> {
        EscapingStringEncoder(&mut self.inner).write_escaped(part);
        self.inner.push_str(" ");
        // println!("escaped | bf:{}/ af:{}/", part, self.inner);

        Ok(())
    }

    #[cfg(feature = "pulldown-cmark")]
    #[inline]
    fn write_html<'b, I: Iterator<Item = Event<'b>>>(&mut self, iter: I) -> Result<(), Self::Error> {
        html::push_html(&mut self.inner, iter);
        // println!("aaaaaaaaa");

        Ok(())
    }

    #[inline]
    fn format_unescaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        use std::fmt::Write;

        // Never fails for a string
        let _ = write!(&mut self.inner, "{} ", display);
        // println!("bbbbbbb");

        Ok(())
    }

    #[inline]
    fn format_escaped<D: fmt::Display>(&mut self, display: D) -> Result<(), Self::Error> {
        use std::fmt::Write;

        // Never fails for a string
        let _ = write!(EscapingStringEncoder(&mut self.inner), "{} ", display);
        // println!("cccccc");

        Ok(())
    }
}



/// 跳过空白字符,
/// 遇到非空白字符时返回 Some(跳过的字符数),
/// 遇到结尾时返回 None
fn skip_blank(s: &str) -> Option<usize> {
    let mut current_idx = 0;
    let slen = s.len();

    while current_idx < slen {
        let c = char_at(s, current_idx);
        let is_not_blank = BLANKET_CHARS.iter().all(|b| *b != c);
        if is_not_blank { break }

        current_idx += 1;
    }

    if current_idx < slen {
        Some(current_idx)
    } else {
        None
    }
}

#[inline]
fn char_at(s: &str, idx: usize) -> u8 {
    *&s[idx..idx + 1].as_bytes()[0]
}

/// stop at blank or end
fn get_token(s: &str) -> Result<Token, SimpleError> {
    let mut current_idx = 0;
    let slen = s.len();

    // ![DEL(xxx)] 处理
    if s.len() >= 6 && "![DEL(" == &s[0..6] {
        current_idx += 6;
        let mut has_end = false;
        while current_idx < slen {
            let c = char_at(s, current_idx);
            let is_blank = BLANKET_CHARS.iter().any(|&b| b == c);
            current_idx += 1;

            if is_blank { 
                break 
            } else if c == b')' {
                let c = char_at(&s[current_idx..], 0);
                if c == b']' {
                    has_end = true;
                    current_idx += 1;
                    break
                }
            }
        }

        if has_end {
            let token = &s[6..current_idx - 2];
            return Ok(Token::DEL(current_idx, token))
        } else {
            Err(SimpleInnerError(" '![DEL(..)' syntax error".to_owned()))?
        }
    } else {
        while current_idx < slen {
            let c = char_at(s, current_idx);
            let is_blank = BLANKET_CHARS.iter().any(|&b| b == c);
            if is_blank { 
                break 
            }
            current_idx += 1;
        }
        let token = &s[0..current_idx];
        return Ok(Token::Normal(current_idx, token))
    }
}