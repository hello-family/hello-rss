use std::borrow::Cow;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    BoxError,
};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

pub struct Json<T>(T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::Json`
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body): (_, Cow<'_, str>) = match rejection {
                    JsonRejection::JsonDataError(err) => (
                        StatusCode::BAD_REQUEST,
                        format!("错误的JSON请求: {}", err).into(),
                    ),
                    JsonRejection::MissingJsonContentType(err) => {
                        (StatusCode::BAD_REQUEST, err.to_string().into())
                    }
                    err => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("未知服务端错误: {}", err).into(),
                    ),
                };

                Err((status, axum::Json(json!({ "error": body }))))
            }
        }
    }
}
