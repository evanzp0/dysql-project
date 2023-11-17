use serde::{Serialize, Deserialize};

use crate::hash_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum SimpleTag {
    Unescaped,
    Section,
    // Tail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct SimpleBlock {
    name: String,
    hash: u64,
    tag: SimpleTag,
    children: u32,
}

impl SimpleBlock {
    #[inline]
    pub fn new(name: &str, tag: SimpleTag, children: u32) -> Self {
        SimpleBlock {
            name: name.to_owned(),
            hash: hash_name(name),
            tag,
            children,
        }
    }
}