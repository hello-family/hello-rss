use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_client")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,
    pub name: String,
    pub expire: i32,
    pub create_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_session::Entity")]
    UserSession,
}

impl Related<super::user_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
