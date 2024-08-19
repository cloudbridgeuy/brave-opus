// See: https://api.search.brave.com/app/documentation/suggest/get-started

//! Suggest API

use crate::query::Query;
use crate::{error, ApiResult, Brave, SuggestSearchApiResponse, SuggestSearchParams};

use super::SUGGEST;

pub trait Api {
    /// # Errors
    ///
    /// Will return `Err` if the POST request fails for some reason.
    fn suggest(
        &self,
        params: &SuggestSearchParams,
        version: Option<&str>,
    ) -> ApiResult<SuggestSearchApiResponse>;
}

impl Api for Brave {
    fn suggest(
        &self,
        params: &SuggestSearchParams,
        version: Option<&str>,
    ) -> ApiResult<SuggestSearchApiResponse> {
        let query_params = params.to_query_params();
        let res = self.query(
            SUGGEST,
            Some(query_params.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()),
            version,
        )?;
        let response: SuggestSearchApiResponse =
            serde_json::from_value(res).map_err(error::Error::DeserializeError)?;
        Ok(response)
    }
}
