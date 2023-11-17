//! Extract name paramters and sql statement from the named sql template.

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
/// let rst = extract_params(sql, SqlDialect::postgres);
/// assert_eq!(
///     ("select * from abc where id=$1 and name=$2 order by id".to_owned(), vec!["id".to_owned(), "name".to_owned()]),
///     rst.unwrap()
/// );
/// 
/// let sql = "select * from abc where id=:id and name=:name order by id";
/// let rst = extract_params(sql, SqlDialect::mysql);
/// assert_eq!(
///     ("select * from abc where id=? and name=? order by id".to_owned(), vec!["id".to_owned(), "name".to_owned()]),
///     rst.unwrap()
/// );
/// ```
pub fn extract_params<'a>(o_sql: &'a str, sql_dial: SqlDialect) -> ParseSqlResult<(String, Vec<&'a str>)> {
    // eprintln!("{:#?}", o_sql);
    let mut r_sql = String::new();
    let mut params: Vec<&'a str> = vec![];

    let mut count = 0;
    let mut start: usize = 0;
    let mut cur = start;
    let end = o_sql.len();

    while cur < end {
        let (found, current_cursor) = char_index(o_sql, cur, vec![':']);

        if found {
            cur = current_cursor;
            count += 1;
            match sql_dial {
                SqlDialect::postgres => {
                    r_sql.push_str(&o_sql[start..cur]);
                    r_sql.push('$');
                    r_sql.push_str(&count.to_string());
                },
                _ => r_sql.push_str(&format!("{}?", &o_sql[start..cur])),
            }
            
            // skip ":" char
            cur += 1;
            start = cur;

            // get named parameter end index
            let err_msg = "not found named parameter after ':'".to_owned();
            if cur == end {
                return Err(ParseSqlError(err_msg))
            } else {
                let (found, current_cursor) = char_index(o_sql, cur, vec![' ', '\n', '\t', ',', ';', '{', ')', '|']);
                if found && current_cursor == cur {
                    return Err(ParseSqlError(err_msg))
                }

                cur = current_cursor;
                let p = &o_sql[start..cur];
                params.push(p);
                start = cur;
            }
        } else {
            let rail_sql = &o_sql[start..end];
            r_sql.push_str(rail_sql);
            break;
        }
    }

    Ok((r_sql, params))
}

///
/// get the index for specified chars in the string slice from begin pos
/// 
fn char_index(s: &str, begin: usize, search_chars:Vec<char>) -> (bool, usize) {
    let end = s.len();
    for i in begin..end {
        let c = &s[i..i+1];
        for j in 0..search_chars.len() {
            let a = &search_chars[j].to_string();
            if c == a {
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
        let rst = extract_params(sql, SqlDialect::postgres);
        assert_eq!(("select * from abc where id=$1 and name=$2".to_owned(), vec!["id", "name"]), rst.unwrap());
    }

    #[test]
    fn test_extract_wrong_parameter() {
        let sql = "select * from abc where id=: id and name=:name order by id";
        let rst = extract_params(sql, SqlDialect::postgres);
        match rst {
            Ok(_) => panic!("Unexpected error"),
            Err(_) => (),
        };

        let sql = "select * from abc where id=:id and name=:";
        let rst = extract_params(sql, SqlDialect::postgres);
        match rst {
            Ok(_) => panic!("Unexpected error"),
            Err(_) => (),
        };
    }
}