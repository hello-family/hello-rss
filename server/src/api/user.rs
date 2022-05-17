use axum::{Extension, Json};
use http::StatusCode;
use sea_orm::DatabaseConnection;

use crate::{
    dto::user::{SignupInput, TokenPayload},
    error::ApiResult,
    service::user::UserService,
    utils::{jwt, validate_payload},
};

pub async fn signup(
    Json(payload): Json<SignupInput>,
    Extension(db): Extension<DatabaseConnection>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&payload)?;
    let user = UserService::signup(payload, &db).await?;
    let token = jwt::sign(user.id)?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn signup_ok() {
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
