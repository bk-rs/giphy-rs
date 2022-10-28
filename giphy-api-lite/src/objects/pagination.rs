//! https://developers.giphy.com/docs/api/schema/#pagination-object

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pagination {
    pub offset: u32,
    pub total_count: Option<u32>,
    pub count: u32,
}
