use serde::{Serialize, Deserialize};

use crate::{Content, simple::simple_section::SimpleSection};

use super::{SimpleValue, simple_block::{SimpleBlock, SimpleTag}, SimpleError};

/// 用于在 SQL 中绑定 DTO 值的简化模版
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleTemplate {
    blocks: Vec<SimpleBlock>,
}

impl SimpleTemplate
{
    /// 生成简化模版
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

        println!("blocks: {:#?}", blocks);

        Self {
            blocks,
        }
    }

    /// 获取对应参数值
    pub fn apply<C>(&self, content: &C) -> Result<SimpleValue, SimpleError>
    where 
        C: Content,
    {
        let section = SimpleSection::new(&self.blocks)
            .with(content);
        section.apply()
    }
}