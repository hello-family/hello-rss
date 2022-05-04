use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct SignupInput {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6, max = 12))]
    username: String,
    #[validate(length(min = 8, max = 20))]
    password: String,
}

pub async fn handle(extract::Json(payload): extract::Json<SignupInput>) -> Json<Value> {
    match payload.validate() {
        Ok(_) => {
            return Json(json!({
                "status": "ok",
                "message": "User created successfully"
            }))
        }
        Err(e) => {
            return Json(json!({
                "status": "error",
                "message": e
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn json() {
        todo!()
        // let app = app();

        // let response = app
        //     .oneshot(
        //         Request::builder()
        //             .method(http::Method::POST)
        //             .uri("/json")
        //             .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        //             .body(Body::from(
        //                 serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
        //             ))
        //             .unwrap(),
        //     )
        //     .await
        //     .unwrap();

        // assert_eq!(response.status(), StatusCode::OK);

        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // let body: Value = serde_json::from_slice(&body).unwrap();
        // assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }
}
