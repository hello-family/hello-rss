use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
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

    pub create_at: DateTimeUtc,

    pub update_at: DateTimeUtc,
}

#[derive(EnumIter, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sea_orm(rs_type = "u16", db_type = "SmallUnsigned")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Inactive,
    #[sea_orm(num_value = 1)]
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
