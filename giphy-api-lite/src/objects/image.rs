//! https://developers.giphy.com/docs/api/schema/#image-object

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Images {
    pub original: OriginalImage,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OriginalImage {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: usize,
    pub webp: Url,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub webp_size: usize,
}
