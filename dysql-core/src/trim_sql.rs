use std::io::Cursor;

use crate::{DySqlError, Kind, ErrorInner};

const BLANKET_CHARS: [u8; 3] = [b' ', b'\n', b'\t'];

enum Token<'a> {
    // (含控制符的 len, token str)
    DEL(usize, &'a str), 
    Normal(usize, &'a str),
}

pub fn trim_sql(sql: &str, sql_buf: Vec<u8>) -> Result<Vec<u8>, DySqlError> {
    use std::io::Write;

    let mut sql_buf = Cursor::new(sql_buf);
    let mut end: usize = 0; // 当前位置
    let sql_len: usize = sql.len();
    let mut trim_token: Option<&str> = None;

    while end < sql_len { 
        if let Some(idx) = skip_blank(&sql[end..sql_len]) {
            end += idx;
        } else {
            break;
        }
        
        let token_len = match get_token(&sql[end..sql_len])? {
            // 如果是 DEL 控制符，则记录需要 trim 的 token，在下一次写入 sql_buf 时过滤字符串
            Token::DEL(len, token) => {
                trim_token = Some(token);
                len
            }
            // 需要输出的 sql token 如果开始位置有需要 DEL 的 token，则跳过此 token 写入 sql_buf,
            // 写入后重置 trim_token 为 None.
            Token::Normal(len, token) => {
                if let Some(tm_token) = trim_token {
                    let trim_len = tm_token.len();
                    if trim_len <= token.len() && tm_token == &token[0..trim_len] {
                        if trim_len < token.len() {
                            write!(sql_buf, "{} ", &token[trim_len..]).expect("trim_sql failed");
                        } else {
                            write!(sql_buf, "{}", &token[trim_len..]).expect("trim_sql failed");
                        }
                    } else {
                        write!(sql_buf, "{} ", token).expect("trim_sql failed");
                    }
                } else {
                    write!(sql_buf, "{} ", token).expect("trim_sql failed");
                }
                trim_token = None;

                len
            }
        };

        end += token_len;
    }

    let mut sql_buf = sql_buf.into_inner();
    if sql_buf.len() > 0 {
        sql_buf.pop();
    }

    Ok(sql_buf)
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
fn get_token(s: &str) -> Result<Token, DySqlError> {
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
            Err(DySqlError(ErrorInner { kind: Kind::ParseSqlError, cause: None, message: Some(" '![DEL(..)' syntax error".to_owned()) }))?
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let sql = 
            "select * from test_user 
            where
            {{#data}}
                ![DEL(and)]
                {{#name}}and name like '%' || :data.name || '%'{{/name}}
                {{#age}}and age > :data.age{{/age}}
                {{?id_rng}}
                    and id in (
                        ![DEL(,)] {{#id_rng}} , {{$value}} {{/id_rng}} 
                    )
                {{/id_rng}}
            {{/data}}
            ";

        let sql_buf: Vec<u8> = Vec::with_capacity(sql.len());
        let sql_buf = trim_sql(sql, sql_buf).unwrap();
        let sql = unsafe { std::str::from_utf8_unchecked(&sql_buf) };

        println!("{}", sql);

        // assert_eq!("update test_user set id = :id , name = :name where age in ( 10, 20, 30 )", sql);
    }
}
