use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .unsigned()
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Username)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Password)
                            .string()
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Email)
                            .string()
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Status)
                            .tiny_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            ).await?;
            manager.create_index(
                Index::create()
                    .name("idx-username")
                    .unique()
                    .table(user::Entity)
                    .col(user::Column::Username)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-email")
                    .unique()
                    .table(user::Entity)
                    .col(user::Column::Email)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
