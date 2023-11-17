use serde::{Serialize, Deserialize};

use crate::{SimpleSection, Content};

use super::{SimpleBlock, SimpleTag};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleTemplate {
    blocks: Vec<SimpleBlock>
}

impl SimpleTemplate {
    pub fn new(source: &str) -> Self
    {
        let param_names: Vec<&str> = source.split(".").collect();
        let param_len = param_names.len();
        let blocks = param_names.iter().enumerate().map(|(i, name)| {
            let children = param_len - i - 1;
            let tag = if children > 0 {
                SimpleTag::Section
            } else {
                SimpleTag::Unescaped
            };
            
            SimpleBlock::new(name, tag, children as u32)
        }).collect();

        Self {
            blocks
        }
    }

    pub fn apply<F, V, Q, C>(&self, binder: F, content: &C)
    where 
        F: FnOnce(V, Q) -> Q,
        C: Content,
    {
        let section = SimpleSection::new(&self.blocks, binder)
            .with(content);
        section.apply();
    }
}