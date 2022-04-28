use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rss_channel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub title: String,
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
    pub update_at: DateTimeUtc,
    pub create_at: DateTimeUtc,
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
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
