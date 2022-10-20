///
/// extract sql and params from raw sql
/// 
/// # Examples
///
/// Basic usage:
/// 
/// ```
/// # use sqlmap::*;
///
/// let sql = "select * from abc where id=:id and name=:name order by id";
/// let rst = extract_params(sql);
/// assert_eq!(
///     Ok(
///         (
///             "select * from abc where id=$1 and name=$2 order by id".to_owned(),
///             vec!["id".to_owned(), "name".to_owned()]
///         ) 
///     ),
///     rst
/// );
/// ```
pub fn extract_params(o_sql: &str) -> Result<(String, Vec<String>), String>{
    // eprintln!("{:#?}", o_sql);
    let mut r_sql = String::new();
    let mut params: Vec<String> = vec![];

    let mut count = 0;
    let mut start: usize = 0;
    let mut cur = start;
    let end = o_sql.len();
    while cur < end {
        let (found, current_cursor) = char_index(o_sql, cur, vec![':']);

        if found {
            cur = current_cursor;
            count += 1;
            r_sql.push_str(&format!("{}${}", &o_sql[start..cur], count));

            // skip ":" char
            cur += 1;
            start = cur;

            // get named param end index
            if cur == end {
                return Err("named parameter format error".to_owned())
            } else {
                let (found, current_cursor) = char_index(o_sql, cur, vec![' ', '\n', '\t']);
                if found && current_cursor == cur{
                    return Err("named parameter format error".to_owned())
                }

                cur = current_cursor;
                let p = &o_sql[start..cur];
                params.push(p.to_string());
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
pub fn char_index(s: &str, begin: usize, search_chars:Vec<char>) -> (bool, usize) {
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
    fn test() {
        let sql = "select * from abc where id=:id and name=:name";
        let rst = extract_params(sql);
        assert_eq!(
            Ok(
                (
                    "select * from abc where id=$1 and name=$2".to_owned(),
                    vec!["id".to_owned(), "name".to_owned()]
                ) 
            ),
            rst
        );
        
        let sql = "select * from abc where id=: id and name=:name order by id";
        let rst = extract_params(sql);
        assert_eq!(
            Err("named parameter format error".to_owned()),
            rst
        );

        let sql = "select * from abc where id=:id and name=:";
        let rst = extract_params(sql);
        assert_eq!(
            Err("named parameter format error".to_owned()),
            rst
        );
    }
}