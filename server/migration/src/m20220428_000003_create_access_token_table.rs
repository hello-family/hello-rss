use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000003_create_access_token_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(access_token::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(access_token::Column::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::ClientId)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::AccessToken)
                            .string()
                            .string_len(128)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::RefreshToken)
                            .string()
                            .string_len(128)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::ExpireAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::CreateAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(access_token::Column::UpdateAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user")
                    .from(access_token::Entity, access_token::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_client")
                    .from(access_token::Entity, access_token::Column::ClientId)
                    .to(client::Entity, client::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(access_token::Entity).to_owned())
            .await
    }
}
