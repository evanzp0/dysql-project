use serde::Serialize;

use crate::PageDto;

mod content;

#[derive(Debug, Serialize, Clone)]
// #[derive(Content)]
pub struct Pagination <T> {
    pub data: Vec<T>,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: u64,
    pub start: u64,
    pub total: u64,
}

impl<T> Pagination<T> {
    pub fn from_dto<Dto>(dto: &PageDto<Dto>, data: Vec<T>) -> Self 
    where
        Dto: Clone
    {
        Self {
            data,
            page_size: dto.page_size,
            page_no: dto.page_no,
            total_page: dto.total_page.expect("Unexpected error"),
            start: dto.start.expect("Unexpected error"),
            total: dto.total.expect("Unexpected error")
        }
    }
}