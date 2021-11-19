//! https://developers.giphy.com/docs/api/schema#gif-object

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::{objects::image::Images, types::rating::Rating};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Gif {
    pub r#type: GifType,
    pub id: String,
    pub slug: String,
    pub url: String,
    pub username: String,
    pub rating: Rating,
    pub images: Images,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GifType {
    Gif,
    Sticker,
    Text,
    Emoji,
}
