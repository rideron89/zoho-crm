//! Various response objects returned from Zoho.

use serde::Deserialize;

/// Wrapper around a successful response using the `get()` method.
#[derive(Debug, Deserialize)]
pub struct ApiGetResponse<T> {
    pub data: Vec<T>,
}

/// Wrapper around a successful response using the `get_many()` method.
///
/// Because Zoho always sends the last page of data after reaching the end, you should use
/// something like the following to determine when to stop fetching:
#[derive(Debug, Deserialize)]
pub struct ApiGetManyResponse<T> {
    pub data: Vec<T>,
    pub info: ApiGetManyResponseInfo,
}

/// Meta data sent back with the `get_many()` method.
#[derive(Debug, Deserialize)]
pub struct ApiGetManyResponseInfo {
    pub count: usize,
    pub more_records: bool,
    pub page: usize,
    pub per_page: usize,
}

/// This is one possible error response that Zoho might send back when requesting a token. If
/// the API response contains an `error` field, it will be treated as an `AuthErrorResponse`
/// and should be handled accordingly.
#[derive(Debug, Deserialize)]
pub struct AuthErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiSuccessResponse {
    pub data: Vec<ApiSuccessResponseDataItem>,
}

#[derive(Debug, Deserialize)]
pub struct ApiSuccessResponseDataItem {
    pub code: String,
    pub details: ResponseDataItemDetails,
    pub message: String,
    pub status: String,
}

// The order of the variants matter here, because `serde` will try to match each variant,
// starting from the top.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseDataItemDetails {
    Success(ResponseDataItemDetailsSuccess),
    Error(ResponseDataItemDetailsError),
}

#[derive(Debug, Deserialize)]
pub struct ResponseDataItemDetailsError {
    pub api_name: Option<String>,
    pub expected_data_type: Option<String>,
    pub index: Option<String>,
}

/// Response details object returned when a record was succesfully insert or updated.
///
/// There are some other fields, shown [here](https://www.zoho.com/crm/developer/docs/api/insert-records.html),
/// but they are ignored for now, for simplicity's sake.
#[derive(Debug, Deserialize)]
pub struct ResponseDataItemDetailsSuccess {
    #[serde(alias = "Modified_Time")]
    pub modified_time: String,

    #[serde(alias = "Created_Time")]
    pub created_time: String,

    pub id: String,
}

/// This is one possible error response that Zoho might send back from an API request. It is
/// different than the response format given back when requesting a token. `code` will be an
/// identifier for the type of error, while the `message` field *might* have more information.
///
/// `status` will return a text status: "error" on error.
///
/// There is also a `data` field we are not capturing.
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    #[allow(dead_code)]
    pub code: String,

    #[allow(dead_code)]
    pub message: String,

    #[allow(dead_code)]
    pub status: String,
}

impl ApiErrorResponse {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("[{}] {}", self.code, self.message)
    }
}
