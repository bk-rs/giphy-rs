//! https://developers.giphy.com/docs/optional-settings/#language-support

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
pub enum Lang {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "cn")]
    ChineseSimplified,
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Lang::English.to_string(), "en");
    }
}
