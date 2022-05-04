use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "favorite")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub rss_item_id: i32,
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
        belongs_to = "super::rss_item::Entity",
        from = "Column::RssItemId",
        to = "super::rss_item::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
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
