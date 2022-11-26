use ramhorns::Content;
use serde::Deserialize;


#[derive(Debug, Content, Deserialize)]
pub struct SortModel {
    field: String,
    sort: String,
}
