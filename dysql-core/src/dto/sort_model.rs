mod content;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
// #[derive(Content)]
pub struct SortModel {
    pub field: String,
    pub sort: String,
}
