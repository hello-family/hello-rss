use entity::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000006_creaate_favorite_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(favorite::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(favorite::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(favorite::Column::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(favorite::Column::RssItemId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(favorite::Column::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_favorite_user_id")
                    .from(favorite::Entity, favorite::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_favorite_rss_item_id")
                    .from(favorite::Entity, favorite::Column::RssItemId)
                    .to(rss_item::Entity, rss_item::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(favorite::Entity).to_owned())
            .await
    }
}
