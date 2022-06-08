use async_graphql::{Enum, SimpleObject};
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub username: String,

    #[sea_orm(unique)]
    pub email: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub password: String,

    pub status: Status,

    pub create_at: DateTime,

    pub update_at: DateTime,
}

#[derive(
    EnumIter, Enum, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy,
)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum Status {
    #[sea_orm(num_value = 0i16)]
    Inactive,
    #[sea_orm(num_value = 1i16)]
    Active,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access_token::Entity")]
    AccessToken,
    #[sea_orm(has_many = "super::rss_channel::Entity")]
    RssChannel,
    #[sea_orm(has_many = "super::rss_item::Entity")]
    RssItem,
}

impl Related<super::access_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccessToken.def()
    }
}

impl Related<super::rss_channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RssChannel.def()
    }
}

impl Related<super::rss_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RssItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_username(username: &str) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(username))
    }

    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}