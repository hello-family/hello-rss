use async_graphql::{Enum, SimpleObject};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject)]
#[sea_orm(table_name = "client")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,
    pub name: ClientType,
    pub expire: i32,
    pub create_at: DateTime,
}

#[derive(
    EnumIter, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq, Enum, Copy, Eq,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(12))")]
pub enum ClientType {
    #[sea_orm(string_value = "Desktop")]
    Desktop,
    #[sea_orm(string_value = "Web")]
    Web,
    #[sea_orm(string_value = "Android")]
    Android,
    #[sea_orm(string_value = "iOS")]
    IOS,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access_token::Entity")]
    AccessToken,
}

impl Related<super::access_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccessToken.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
