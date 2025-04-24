use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

/// A standardized API response format.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

/// A standardized API response format for successful and failed responses.
/// This struct is used to wrap the response data and provide a consistent format for all API responses.
/// It includes a status code, a message, and optional data.
/// The `status` field indicates the HTTP status code of the response.
/// The `message` field contains a human-readable message describing the result of the request.
/// The `data` field contains the actual data returned by the API, if any.
impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a success response with default message "success".
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    /// Create a success response with a custom message.
    pub fn success_with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            status: 200,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Create a failure response with no data.
    pub fn failure(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            data: None,
        }
    }
}

/// A wrapper struct for the API response.
/// This struct is used to convert the API response into a format that can be returned by Axum.
/// It implements the `IntoResponse` trait, which allows it to be used as a response in Axum handlers.

#[derive(Deserialize, Debug)]
pub struct RestApiResponse<T: Serialize>(pub ApiResponse<T>);

/// A wrapper for the API response that implements `IntoResponse`.
/// This struct is used to convert the API response into a format that can be returned by Axum.
/// It implements the `IntoResponse` trait, which allows it to be used as a response in Axum handlers.
impl<T: Serialize> RestApiResponse<T> {
    /// Return a successful response with default message.
    pub fn success(data: T) -> Self {
        Self(ApiResponse::success(data))
    }

    /// Return a successful response with a custom message.
    pub fn success_with_message(message: impl Into<String>, data: T) -> Self {
        Self(ApiResponse::success_with_message(message, data))
    }

    /// Return a failed response with a status code and message.
    pub fn failure(status: u16, message: impl Into<String>) -> Self {
        Self(ApiResponse::failure(status, message))
    }
}

impl<T: Serialize> IntoResponse for RestApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
