use chrono::Utc;
use entity::{
    client::{self, ClientType},
    sea_orm::{EntityTrait, Set},
};
use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(client::Column::Expire).integer().not_null())
                    .col(
                        ColumnDef::new(client::Column::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        let desktop = client::ActiveModel {
            id: Set(1i16.to_owned()),
            name: Set(ClientType::Desktop.to_owned()),
            expire: Set(0.to_owned()),
            create_at: Set(Utc::now().naive_utc().to_owned()),
        };
        let web = client::ActiveModel {
            id: Set(2i16.to_owned()),
            name: Set(ClientType::Web.to_owned()),
            expire: Set(7200.to_owned()),
            create_at: Set(Utc::now().naive_utc().to_owned()),
        };
        let android = client::ActiveModel {
            id: Set(3i16.to_owned()),
            name: Set(ClientType::Android.to_owned()),
            expire: Set(0.to_owned()),
            create_at: Set(Utc::now().naive_utc().to_owned()),
        };
        let ios = client::ActiveModel {
            id: Set(4i16.to_owned()),
            name: Set(ClientType::IOS.to_owned()),
            expire: Set(0.to_owned()),
            create_at: Set(Utc::now().naive_utc().to_owned()),
        };
        client::Entity::insert_many(vec![desktop, web, android, ios])
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(client::Entity).to_owned())
            .await
    }
}
