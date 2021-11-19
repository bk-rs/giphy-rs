//! https://developers.giphy.com/docs/api/endpoint#trending

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    endpoints::{
        common::{EndpointError, EndpointRet},
        URL_BASE, USER_AGENT_VALUE,
    },
    objects::{gif::Gif, meta::Meta, pagination::Pagination, ResponseBodyErrJson},
    types::rating::Rating,
};

#[derive(Debug, Clone)]
pub struct Emoji {
    pub api_key: String,
    //
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub rating: Option<Rating>,
    pub random_id: Option<String>,
}

impl Emoji {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        Self {
            api_key: api_key.as_ref().to_owned(),
            limit: Default::default(),
            offset: Default::default(),
            rating: Default::default(),
            random_id: Default::default(),
        }
    }
}

impl Endpoint for Emoji {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<EmojiResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("{}/emoji", URL_BASE);
        let mut url = Url::parse(url.as_str()).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("api_key", &self.api_key);

        if let Some(limit) = &self.limit {
            url.query_pairs_mut()
                .append_pair("limit", limit.to_string().as_str());
        }
        if let Some(offset) = &self.offset {
            url.query_pairs_mut()
                .append_pair("offset", offset.to_string().as_str());
        }
        if let Some(rating) = &self.rating {
            url.query_pairs_mut()
                .append_pair("rating", rating.to_string().as_str());
        }
        if let Some(random_id) = &self.random_id {
            url.query_pairs_mut().append_pair("random_id", random_id);
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(USER_AGENT, USER_AGENT_VALUE)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            )),
            StatusCode::TOO_MANY_REQUESTS => Ok(EndpointRet::RateLimitIsReached),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmojiResponseBodyOkJson {
    pub data: Vec<Gif>,
    pub pagination: Pagination,
    pub meta: Meta,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body_ok_json() {
        match serde_json::from_str::<EmojiResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/emoji_ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.len(), 50);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
