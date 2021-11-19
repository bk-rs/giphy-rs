//! https://developers.giphy.com/docs/api/schema/#meta-object

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Meta {
    pub msg: String,
    pub status: u32,
    pub response_id: String,
}
