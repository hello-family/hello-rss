use entity::{client::ClientType, prelude::User};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupInput {
    pub client: String,
    #[validate(length(min = 4, max = 10))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    pub client: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
}
