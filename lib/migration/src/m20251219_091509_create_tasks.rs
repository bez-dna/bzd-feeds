use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Tasks;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Tasks::Table)
                    .col(uuid(Tasks::TaskId).primary_key())
                    .col(json_binary(Tasks::Payload))
                    .col(timestamp_null(Tasks::LockedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}
