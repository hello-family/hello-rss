use entity::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000005_create_rss_item_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(rss_item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(rss_item::Column::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(rss_item::Column::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(rss_item::Column::ChannelId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(rss_item::Column::Title).string().not_null())
                    .col(ColumnDef::new(rss_item::Column::Link).string().not_null())
                    .col(ColumnDef::new(rss_item::Column::Description).string())
                    .col(ColumnDef::new(rss_item::Column::Author).string())
                    .col(ColumnDef::new(rss_item::Column::Categories).json())
                    .col(ColumnDef::new(rss_item::Column::Comments).string())
                    .col(
                        ColumnDef::new(rss_item::Column::Guid)
                            .string()
                            .string_len(128)
                            .unique_key(),
                    )
                    .col(ColumnDef::new(rss_item::Column::GuidIsPermalink).boolean())
                    .col(ColumnDef::new(rss_item::Column::PubDate).timestamp())
                    .col(ColumnDef::new(rss_item::Column::Source).string())
                    .col(ColumnDef::new(rss_item::Column::Content).string())
                    .col(
                        ColumnDef::new(rss_item::Column::IsRead)
                            .default(false)
                            .boolean(),
                    )
                    .col(ColumnDef::new(rss_item::Column::ReadAt).timestamp())
                    .col(
                        ColumnDef::new(rss_item::Column::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(rss_item::Column::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user")
                    .from(rss_item::Entity, rss_item::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_rss_channel")
                    .from(rss_item::Entity, rss_item::Column::ChannelId)
                    .to(rss_channel::Entity, rss_channel::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(rss_item::Entity).to_owned())
            .await
    }
}
