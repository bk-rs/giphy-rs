//! https://developers.giphy.com/docs/optional-settings/#rating

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    G,
    PG,
    #[serde(rename = "pg-13")]
    PG13,
    R,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Rating::PG.to_string(), "pg");
        assert_eq!(Rating::PG13.to_string(), "pg-13");
    }
}
