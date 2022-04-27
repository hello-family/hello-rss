use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub username: String,
    #[serde(skip_deserializing)]
    pub password: String,
    pub email: String,
    pub status: Status,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(EnumIter, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Inactive,
    #[sea_orm(num_value = 1)]
    Active,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
