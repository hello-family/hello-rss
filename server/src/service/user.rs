use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    dto::user::{LoginInput, RegisterInput},
    error::{Error, Result},
    utils::encryption,
};
use entity::{prelude::User, user};

pub struct UserService;

impl UserService {
    pub async fn login(input: LoginInput, db: &DatabaseConnection) -> Result<user::Model> {
        let entity: user::Model = User::find()
            .filter(user::Column::Username.contains(&input.username))
            .one(db)
            .await
            .unwrap()
            .unwrap()
            .into();
        if encryption::verify_password(input.password, entity.password.to_owned()).await? {
            Ok(entity)
        } else {
            Err(Error::WrongPassword)
        }
    }

    pub async fn sign_up(input: RegisterInput, pool: &DatabaseConnection) -> Result<User> {
        if User::find_by_email(&input.email, &pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }
        if User::find_by_name(&input.name, &pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(User::create(data, &pool).await?)
    }
}
