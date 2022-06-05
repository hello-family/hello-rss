use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "rss_channel")]
#[graphql(concrete(name = "RssChannel", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub docs: Option<String>,
    pub image: Option<String>,
    pub skip_hours: Json,
    pub skip_days: Json,
    pub ttl: Option<i32>,
    #[sea_orm(default = "0")]
    pub unread: i32,
    pub update_at: DateTime,
    pub create_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "super::rss_item::Entity")]
    RssItem,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::rss_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RssItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
