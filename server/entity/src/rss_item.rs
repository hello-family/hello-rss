use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rss_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub channel_id: i32,
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Json,
    pub comments: Option<String>,
    pub guid: Option<String>,
    pub guid_is_permalink: Option<bool>,
    pub pub_date: Option<String>,
    pub source: Option<String>,
    pub content: Option<String>,
    #[sea_orm(default = "false")]
    pub is_read: bool,
    pub read_at: Option<DateTimeUtc>,
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
    #[sea_orm(
        belongs_to = "super::rss_channel::Entity",
        from = "Column::ChannelId",
        to = "super::rss_channel::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Channel,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
