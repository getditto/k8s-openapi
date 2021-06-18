// Generated from definition io.k8s.ListResponse

/// The common response type for all list API operations.
#[cfg(feature = "api")]
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub enum ListResponse<T> where T: crate::serde::de::DeserializeOwned + crate::ListableResource {
    Ok(crate::List<T>),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl<T> crate::Response for ListResponse<T> where T: crate::serde::de::DeserializeOwned + crate::ListableResource {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListResponse::Other(result), read))
            },
        }
    }
}
