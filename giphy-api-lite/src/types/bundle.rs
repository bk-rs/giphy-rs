//! https://developers.giphy.com/docs/optional-settings#renditions-on-demand

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Bundle {
    MessagingNonClips,
    StickerLayering,
    LowBandwidth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Bundle::MessagingNonClips.to_string(), "messaging_non_clips");
    }
}
