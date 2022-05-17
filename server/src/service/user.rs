use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ActiveModelTrait};

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

    pub async fn sign_up(input: RegisterInput, db: &DatabaseConnection) -> Result<user::Model> {
        let email_exist = User::find()
        .filter(user::Column::Email.contains(&input.email))
        .one(db)
        .await
        .is_ok();
        if email_exist {
            return Err(Error::DuplicateUserEmail);
        }
        let username_exist = User::find()
        .filter(user::Column::Username.contains(&input.username))
        .one(db)
        .await
        .is_ok();
        if username_exist {
            return Err(Error::DuplicateUserName);
        }
        let model = user::ActiveModel {
            username: Set(input.username),
            email: Set(input.email),
            password: Set(encryption::hash_password(input.password).await?),
            create_at: Set(Utc::now()),
            update_at: Set(Utc::now()),
            ..Default::default() 
        };
        
        let model: user::Model = model.insert(db).await.unwrap();
        Ok(model)
    }
}
