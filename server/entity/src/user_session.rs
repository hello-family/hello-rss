use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_session")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,
    pub user_id: i32,
    pub client_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_at: DateTimeUtc,
    pub create_at: DateTimeUtc,
    pub update_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_client::Entity",
        from = "Column::ClientId",
        to = "super::user_client::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserClient,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user_client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserClient.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
