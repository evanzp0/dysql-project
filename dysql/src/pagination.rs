use ramhorns_ext::Content;
use serde::{Deserialize, Serialize};

use crate::SortModel;

#[derive(Content, Debug, Deserialize)]
pub struct PageDto <T> {
    pub data: T,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: Option<u64>,
    pub start: Option<u64>,
    pub total: Option<u64>,
    pub is_sort: Option<bool>,
    pub sort_model: Option<Vec<SortModel>>,
}

#[derive(Content, Debug, Serialize)]
pub struct Pagination <T> {
    pub data: Vec<T>,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: u64,
    pub start: u64,
    pub total: u64,
}

impl<T> PageDto<T>
{
    pub fn new(page_size: u64, page_no: u64, data: T) -> Self {
        Self {
            page_size,
            page_no,
            data,
            total_page: None,
            start: None,
            total: None,
            is_sort: None,
            sort_model: None
        }
    }

    pub fn new_with_sort(page_size: u64, page_no: u64, data: T, sort_model: Vec<SortModel>) -> Self {
        Self {
            page_size,
            page_no,
            data,
            total_page: None,
            start: None,
            total: None,
            is_sort: Some(true),
            sort_model: Some(sort_model),
        }
    }

    pub fn init(&mut self, total: u64) {
        let total_page = self.total_page(total);
        if total_page <= self.page_no {
            if total_page > 0 {
                self.page_no = total_page - 1;
            } else {
                self.page_no = 0;
            }
        }

        // println!("total: {}, total page: {}, page no: {}", total, total_page, self.page_no);

        let (start, page_no) = self.start(total);

        self.start = Some(start);
        self.total_page = Some(total_page);
        self.total = Some(total);
        self.page_no = page_no;

        self.init_sort();
    }

    pub fn init_sort(&mut self) -> &mut Self {
        if let Some(sm) = &self.sort_model {
            if sm.len() > 0 {
                self.is_sort = Some(true);
            } else {
                self.is_sort = None;
            }
        } else {
            self.is_sort = None;
        }

        self
    }

    fn total_page(&self, total: u64) -> u64 {
        let count: f64 = (total as f64) / self.page_size as f64;
        let count = count.ceil() as u64;
        
        count
    }

    fn start(&self, total: u64) -> (u64, u64) {
        let mut page_no = self.page_no;
        let mut start = self.start_of_page(page_no);

        if start as u64 > total {
            page_no = if self.total_page(total) > 0 {
                self.total_page(total) - 1
            } else {
                0
            };
            start = self.start_of_page(page_no);
        }

        (start, page_no)
    }

    fn start_of_page(&self, page_no: u64) -> u64 {
        self.page_size * page_no
    }
}

impl<T> Pagination<T> {
    pub fn from_dto<Dto>(dto: &PageDto<Dto>, data: Vec<T>) -> Self {
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
