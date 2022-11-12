use ramhorns::Content;
use serde::{Deserialize, Serialize};

#[derive(Content, Debug, Deserialize)]
pub struct PageDto <T> {
    pub dto: T,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: Option<u64>,
    pub start: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Content, Debug, Serialize)]
pub struct Pagination <T> {
    pub data: T,
    pub page_size: u64,
    pub page_no: u64,
    pub total_page: u64,
    pub start: u64,
    pub total: u64,
}

impl<T> PageDto<T>
{
    pub fn new(page_size: u64, page_no: u64, dto: T) -> Self {
        Self {
            page_size,
            page_no,
            dto,
            total_page: None,
            start: None,
            total: None,
        }
    }

    pub fn init(&mut self, total: u64) {
        let (start, page_no) = self.start(total);
        let total_page = self.total_page(total);

        self.start = Some(start);
        self.total_page = Some(total_page);
        self.total = Some(total);
        self.page_no = page_no;
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
            page_no = self.total_page(total);
            start = self.start_of_page(page_no);
            
        }

        (start, page_no)
    }

    fn start_of_page(&self, page_no: u64) -> u64 {
        self.page_size * (page_no - 1) 
    }
}

impl<T> Pagination<T> {
    pub fn from_dto<Dto>(dto: &PageDto<Dto>, data: T) -> Self {
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
