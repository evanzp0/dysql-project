mod content;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
// #[derive(Content)]
pub struct SortModel {
    pub field: String,
    pub sort: String,
}
