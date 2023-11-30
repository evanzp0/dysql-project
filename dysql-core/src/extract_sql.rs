//! Extract name paramters and sql statement from the named sql template.

use std::io::Cursor;

use crate::{sql_dialect::SqlDialect, error::{ParseSqlResult, ParseSqlError}};

///
/// extract sql and params from raw sql
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```ignore
///
/// let sql = "select * from abc where id=:id and name=:name order by id";
/// let mut buf = Vec::<u8>::with_capacity(sql.len());
/// let rst = extract_params_buf(sql, &mut buf, SqlDialect::postgres);
/// assert_eq!(
///     ("select * from abc where id=$1 and name=$2 order by id".to_owned(), vec!["id".to_owned(), "name".to_owned()]),
///     rst.unwrap()
/// );
/// ```
pub fn extract_params_buf<'a>(o_sql: &'a str, sql_buf: &mut Vec<u8>, sql_dial: SqlDialect) -> ParseSqlResult<Vec<&'a str>> {
    use std::io::Write;

    let mut sql_buf = Cursor::new(sql_buf);
    let mut params = vec![];

    let mut count = 0;
    let mut start: usize = 0;
    let mut cur = start;
    let end = o_sql.len();

    while cur < end {
        let (found, current_cursor) = char_index(o_sql, cur, &[b':']);

        if found {
            cur = current_cursor;
            count += 1;
            match sql_dial {
                SqlDialect::postgres => {
                    write!(&mut sql_buf, "{}{}{}", &o_sql[start..cur], "$", &count).unwrap();
                },
                _ => write!(&mut sql_buf, "{}?",&o_sql[start..cur]).unwrap(),
            }
            
            // skip ":" char
            cur += 1;
            start = cur;

            // get named parameter end index
            let err_msg = "not found named parameter after ':'".to_owned();
            if cur == end {
                return Err(ParseSqlError(err_msg))
            } else {
                let (found, current_cursor) = char_index(o_sql, cur, &[b' ', b'\n', b'\t', b',', b';', b'{', b')', b'|']);
                if found && current_cursor == cur {
                    return Err(ParseSqlError(err_msg))
                }

                cur = current_cursor;
                let p = &o_sql[start..cur];
                // params.push(p.to_owned());
                params.push(p);
                start = cur;
            }
        } else {
            write!(&mut sql_buf, "{}", &o_sql[start..end]).unwrap();
            break;
        }
    }

    Ok(params)
}

///
/// get the index for specified chars in the string slice from begin pos
/// 
fn char_index(s: &str, begin: usize, search_chars: &[u8]) -> (bool, usize) {
    let end = s.len();
    for i in begin..end {
        let c = &s[i..i+1];
        for j in 0..search_chars.len() {
            let a = search_chars[j];
            if c.as_bytes()[0] == a {
                return (true, i)
            }
        }
    }
    
    (false, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sql() {
        let sql = "select * from abc where id=:id and name=:name";
        let mut buf = Vec::with_capacity(sql.len());
        let rst = extract_params_buf(sql, &mut buf, SqlDialect::postgres);
        assert_eq!("select * from abc where id=$1 and name=$2", std::str::from_utf8(&buf).unwrap());
        assert_eq!(vec!["id", "name"], rst.unwrap());
    }

    #[test]
    fn test_extract_wrong_parameter() {
        let sql = "select * from abc where id=: id and name=:name order by id";
        let mut buf = Vec::with_capacity(sql.len());
        let rst = extract_params_buf(sql, &mut buf, SqlDialect::postgres);
        match rst {
            Ok(_) => panic!("Unexpected error"),
            Err(_) => (),
        };

        let sql = "select * from abc where id=:id and name=:";
        let mut buf = Vec::with_capacity(sql.len());
        let rst = extract_params_buf(sql, &mut buf, SqlDialect::postgres);
        match rst {
            Ok(_) => panic!("Unexpected error"),
            Err(_) => (),
        };
    }
}