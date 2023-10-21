use ramhorns_ext::Content;
use serde::Deserialize;


#[derive(Debug, Content, Deserialize)]
pub struct SortModel {
    pub field: String,
    pub sort: String,
}
