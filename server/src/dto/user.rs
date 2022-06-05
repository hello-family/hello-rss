use entity::{client::ClientType, user};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupInput {
    pub client: ClientType,
    #[validate(length(min = 4, max = 10))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    pub client: ClientType,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
}
