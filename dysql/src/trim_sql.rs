use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::{DySqlError, Kind, ErrorInner};

const BLANKET_CHARS: [char; 3] = [' ', '\n', '\t'];

#[derive(Debug, Clone)]
pub struct SqlNode<'a> {
    pub token: SqlNodeData<'a>,
    pub next: Option<Rc<RefCell<SqlNode<'a>>>>,
    pub previous: Option<Weak<RefCell<SqlNode<'a>>>>,
}

#[derive(Debug, Clone)]
pub enum SqlNodeData<'a> {
    Str(&'a str),
    FDel{literal: &'a str, data: &'a str},
    BDel{literal: &'a str, data: &'a str},
}

#[derive(Debug)]
pub struct SqlNodeLinkList<'a> {
    pub sql: &'a str,
    pub first: Option<Rc<RefCell<SqlNode<'a>>>>,
    pub last: Option<Weak<RefCell<SqlNode<'a>>>>,
}

impl<'a> SqlNodeLinkList<'a> {
    pub fn new(sql: &'a str) -> Self {
        Self { sql, first: None, last: None }
    }

    pub fn init(&mut self) -> &mut Self {
        let mut start;
        let mut end = 0;
        let sql_len = self.sql.len();

        while end < sql_len {
            if let Some(idx) = skip_blank(&self.sql[end..sql_len]) {
                end += idx;
                start = end;
            } else {
                break;
            }
            
            end += stop_boe(&self.sql[end..sql_len]);

            let token = &self.sql[start..end];
            let token = SqlNodeData::parse(token).unwrap();
            let node = SqlNode::new(token, None, None);
            let node = Rc::new(RefCell::new(node));

            if let None = self.first {
                self.first = Some(node.clone());
                self.last = Some(Rc::downgrade(&node));
            } else {
                if let Some(last) = &mut self.last {
                    {
                        let last = last.upgrade().expect("Unexpected error");
                        let mut tmp_last = (*last).borrow_mut();
                        tmp_last.next = Some(node.clone());

                        (*node).borrow_mut().previous = Some(Rc::downgrade(&last));
                    }
                    self.last = Some(Rc::downgrade(&node));
                } else {
                    panic!("Unexpected error: SqlNodeLinkList last node is none");
                }
            }
        }

        self
    }

    pub fn trim(&mut self) -> &mut Self{
        let mut cursor = self.first.clone();

        while let Some(c_node) = cursor {
            let token = (*c_node).borrow().token.clone();

            match token {
                SqlNodeData::Str(_) => (),
                SqlNodeData::FDel { literal: _, data } => {
                    let i_cursor = c_node.clone();
                    let mut i_cursor = (*i_cursor).borrow().next.clone();

                    while let Some(i_node) = i_cursor {
                        let token = (*i_node).borrow().token.clone();
                        match token {
                            SqlNodeData::Str(s_data) => {
                                if s_data.len() > 0 {
                                    if s_data.to_uppercase().starts_with(&data.to_uppercase()) {
                                        (*i_node).borrow_mut().token = SqlNodeData::Str(&s_data[data.len()..s_data.len()]);
                                    }
                                    break;
                                }
                            },
                            _ => (),
                        }

                        i_cursor = (*i_node).borrow().next.clone();
                    }
                },
                SqlNodeData::BDel { literal: _, data } => {
                    let i_cursor = c_node.clone();
                    let mut i_cursor = (*i_cursor).borrow().previous.clone();

                    while let Some(i_node) = i_cursor {
                        let i_node = i_node.upgrade().expect("Unexpected error");
                        let token = (*i_node).borrow().token.clone();
                        match token {
                            SqlNodeData::Str(s_data) => {
                                if s_data.len() > 0 {
                                    if s_data.to_uppercase().ends_with(&data.to_uppercase()) {
                                        (*i_node).borrow_mut().token = SqlNodeData::Str(&s_data[0..s_data.len() - data.len()]);
                                    }
                                    break;
                                }
                            },
                            _ => (),
                        }

                        i_cursor = (*i_node).borrow().previous.clone();
                    }
                },
            }

            cursor = (*c_node).borrow().next.clone();
        }

        self
    }

    pub fn to_string(&self) -> String {
        let mut string_buf = String::new();
        let mut cursor = self.first.clone();

        while let Some(c_node) = cursor.clone() {
            let c_node = (*c_node).borrow();
            if let SqlNodeData::Str(s) = c_node.token {
                string_buf.push_str(s);

                if let Some(i_node) = c_node.next.clone() {
                    let token = (*i_node).borrow().token.clone();
                    match token {
                        SqlNodeData::Str(s_data) => {
                            if s_data.len() > 0 {
                                string_buf.push(' ');
                            }
                        },
                        _ => (),
                    }
                }
            } else {
                if let Some(i_node) = c_node.next.clone() {
                    let token = (*i_node).borrow().token.clone();
                    match token {
                        SqlNodeData::Str(s_data) => {
                            if s_data.len() > 0 {
                                string_buf.push(' ');
                            }
                        },
                        _ => (),
                    }
                }
            }

            let tmp = cursor.clone().expect("");
            cursor = (*tmp).borrow().next.clone();
        }

        string_buf
    }
}

impl<'a> SqlNode<'a> {
    pub fn new(token: SqlNodeData<'a>, next: Option<Rc<RefCell<SqlNode<'a>>>>, previous: Option<Weak<RefCell<SqlNode<'a>>>>,) -> Self {
        Self {
            token,
            next,
            previous,
        }
    }
}

impl<'a> SqlNodeData<'a> {
    pub fn parse(token: &'a str) -> Result<SqlNodeData, DySqlError> {
        let tlen = token.len();
        let rst = if token.starts_with("![F_DEL(") {
            let tmp = &token[tlen - 2 .. tlen];
            
            if tmp == ")]" {
                SqlNodeData::FDel{ literal: token, data: &token[8..tlen - 2] }
            } else {
                return Err(DySqlError(ErrorInner::new(Kind::ParseSqlError, None)))
            }
        } else if token.starts_with("![B_DEL(") {
            
            let tmp = &token[tlen - 2 .. tlen];
            if tmp == ")]" {
                SqlNodeData::BDel { literal: token, data: &token[8..tlen - 2] }
            } else {
                return Err(DySqlError(ErrorInner::new(Kind::ParseSqlError, None)))
            }
        } else {
            SqlNodeData::Str(token)
        };

        Ok(rst)
    }
} 

fn char_at(s: &str, idx: usize) -> char {
    s[idx..idx + 1]
        .char_indices()
        .next()
        .expect("Unexpected error")
        .1
}

fn skip_blank(s: &str) -> Option<usize> {
    let mut current_idx = 0;
    let slen = s.len();

    while current_idx < slen {
        let c = char_at(s, current_idx);
        let is_not_blank = BLANKET_CHARS.iter().all(|&b| b != c);
        if is_not_blank { break }

        current_idx += 1;
    }

    if current_idx < slen {
        Some(current_idx)
    } else {
        None
    }
}

/// stop at blank or end
fn stop_boe(s: &str) -> usize {
    let mut current_idx = 0;
    let slen = s.len();

    while current_idx < slen {
        let c = char_at(s, current_idx);
        let is_not_blank = BLANKET_CHARS.iter().all(|&b| b != c);
        if !is_not_blank { break }

        current_idx += 1;
    }

    current_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut node_list = SqlNodeLinkList::new(
            "update test_user
            set id = :id,
                name = :name,
                ![B_DEL(,)]
            where
                ![F_DEL(and)]
                and age in (
                    10,
                    20,
                    30,
                    ![B_DEL(,)]
                )
            "
        );
        node_list.init();
        let rst = node_list.trim().to_string();

        assert_eq!("update test_user set id = :id, name = :name where age in ( 10, 20, 30 )", &rst);
    }
}
