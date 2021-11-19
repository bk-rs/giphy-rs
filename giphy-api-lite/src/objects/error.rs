use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de() {
        match serde_json::from_str::<Error>(include_str!(
            "../../tests/response_body_json_files/err_invalid_api_key.json"
        )) {
            Ok(_err_json) => {}
            Err(err) => panic!("{}", err),
        }
    }
}
