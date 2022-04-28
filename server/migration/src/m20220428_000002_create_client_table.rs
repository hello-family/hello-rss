use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220428_000002_create_client_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(client::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(client::Column::Id)
                            .small_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(client::Column::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::Expire)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::CreateAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(client::Entity).to_owned())
            .await
    }
}
