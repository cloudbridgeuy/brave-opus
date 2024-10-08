// See: https://api.search.brave.com/app/documentation/web-search/get-started

//! Web Search API

use crate::query::Query;
use crate::{error, ApiResult, Brave, WebSearchApiResponse, WebSearchParams};

use super::WEB_SEARCH;

pub trait Api {
    /// # Errors
    ///
    /// Will return `Err` if the POST request fails for some reason.
    fn search(
        &self,
        params: &WebSearchParams,
        version: Option<&str>,
    ) -> ApiResult<WebSearchApiResponse>;
}

impl Api for Brave {
    fn search(
        &self,
        params: &WebSearchParams,
        version: Option<&str>,
    ) -> ApiResult<WebSearchApiResponse> {
        let query_params = params.to_query_params();
        let res = self.query(
            WEB_SEARCH,
            Some(query_params.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()),
            version,
        )?;
        let response: WebSearchApiResponse =
            serde_json::from_value(res).map_err(error::Error::DeserializeError)?;
        Ok(response)
    }
}
