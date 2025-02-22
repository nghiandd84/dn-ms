use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, Uri},
    response::{IntoResponse, Response},
    BoxError, Error,
};

use serde::de::DeserializeOwned;
use serde_qs::Config;

#[derive(Debug, Clone, Copy, Default)]
pub struct FilterParams<T>(pub T);

impl<T, S> FromRequestParts<S> for FilterParams<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = FilterPramsRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_uri(&parts.uri)
    }
}

impl<T> FilterParams<T>
where
    T: DeserializeOwned,
{
    pub fn try_from_uri(value: &Uri) -> Result<Self, FilterPramsRejection> {
        let query = value.query().unwrap_or_default();
        let qs_non_strict = Config::new(5, false);
        let qs_params: Result<T, serde_qs::Error> = qs_non_strict.deserialize_str(&query);

        qs_params
            .map(FilterParams)
            .map_err(|e| FilterPramsRejection::new(e, StatusCode::BAD_REQUEST))
    }
}

/// Rejection type for extractors that deserialize query strings
#[derive(Debug)]
pub struct FilterPramsRejection {
    error: axum::Error,
    status: StatusCode,
}

impl std::fmt::Display for FilterPramsRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to deserialize filter params string. Error: {}",
            self.error,
        )
    }
}

impl FilterPramsRejection {
    /// Create new rejection
    pub fn new<E>(error: E, status: StatusCode) -> Self
    where
        E: Into<BoxError>,
    {
        FilterPramsRejection {
            error: Error::new(error),
            status,
        }
    }
}
impl IntoResponse for FilterPramsRejection {
    fn into_response(self) -> Response {
        let mut res = self.to_string().into_response();
        *res.status_mut() = self.status;
        res
    }
}

impl std::error::Error for FilterPramsRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Uri;
    use serde::Deserialize;

    use crate::filter_param::FilterParams;

    use shared_shared_data_core::{filter::FilterParam, filter_deserialize::*};

    #[test]
    fn test_one() {
        #[derive(Deserialize, Debug)]
        struct ChildParams {
            #[serde(
                default = "default_none_string",
                deserialize_with = "deserialize_filter_from_string"
            )]
            pub field_one: Option<FilterParam<String>>,
            #[serde(
                default = "default_none_u32",
                deserialize_with = "deserialize_filter_from_u32"
            )]
            pub field_two: Option<FilterParam<u32>>,
        }

        #[derive(Deserialize, Debug)]
        struct ParentParams {
            #[serde(
                default = "default_none_string",
                deserialize_with = "deserialize_filter_from_string"
            )]
            pub field_one: Option<FilterParam<String>>,
            pub child: Option<ChildParams>,
        }
        let uri: Uri = "http://example.com/path?field_one=li|hello&child[field_one]=eq|nghia&child[field_two]=li|42"
            .parse()
            .unwrap();
        let result: FilterParams<ParentParams> = FilterParams::try_from_uri(&uri).unwrap();
        println!("result: {:?}", result);
    }
}
