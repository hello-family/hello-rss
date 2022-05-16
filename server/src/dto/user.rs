use entity::{client::ClientType, prelude::User};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
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

#[derive(Debug)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
    pub client: ClientType,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}
