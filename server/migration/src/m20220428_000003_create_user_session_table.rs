use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000003_create_user_session_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user_session::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_session::Column::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::ClientId)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::AccessToken)
                            .string()
                            .string_len(128)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::RefreshToken)
                            .string()
                            .string_len(128)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::ExpireAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::CreateAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_session::Column::UpdateAt)
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
                    .from(user_session::Entity, user_session::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_client")
                    .from(user_session::Entity, user_session::Column::ClientId)
                    .to(user_client::Entity, user_client::Column::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user_session::Entity).to_owned())
            .await
    }
}
