use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    dto::user::{LoginInput, SignupInput},
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

    pub async fn signup(input: SignupInput, db: &DatabaseConnection) -> Result<user::Model> {
        let email_exist = User::find()
            .filter(user::Column::Email.contains(&input.email))
            .one(db)
            .await
            .unwrap()
            .is_some();
        if email_exist {
            return Err(Error::DuplicateUserEmail);
        }
        let username_exist = User::find()
            .filter(user::Column::Username.contains(&input.username))
            .one(db)
            .await
            .unwrap()
            .is_some();
        if username_exist {
            return Err(Error::DuplicateUserName);
        }
        let model = user::ActiveModel {
            username: Set(input.username),
            email: Set(input.email),
            password: Set(encryption::hash_password(input.password).await?),
            create_at: Set(Utc::now().to_owned()),
            update_at: Set(Utc::now().to_owned()),
            status: Set(user::Status::Inactive),
            ..Default::default()
        };
        let model: user::Model = model.insert(db).await.unwrap();
        Ok(model)
    }
}
