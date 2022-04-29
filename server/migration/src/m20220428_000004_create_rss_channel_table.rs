use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000004_create_rss_channel_table"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(rss_channel::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(rss_channel::Column::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Url)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Title)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Link)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Description)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Language)
                            .string()
                            .string_len(20)
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::PubDate)
                            .timestamp()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::LastBuildDate)
                            .timestamp()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Docs)
                            .string()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Image)
                            .string()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::SkipHours)
                            .json()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::SkipDays)
                            .json()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Ttl)
                            .integer()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::Unread)
                            .integer()
                            .default(0)
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::CreateAt)
                            .timestamp()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(rss_channel::Column::UpdateAt)
                            .timestamp()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user")
                    .from(rss_channel::Entity, rss_channel::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(rss_channel::Entity).to_owned())
            .await
    }
}
