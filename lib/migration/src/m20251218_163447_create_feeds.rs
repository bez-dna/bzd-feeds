use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Feeds;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Feeds::Table)
                    .col(uuid(Feeds::FeedId).primary_key())
                    .col(uuid(Feeds::UserId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("feeds_user_id_udx")
                    .unique()
                    .table(Feeds::Table)
                    .col(Feeds::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feeds::Table).to_owned())
            .await
    }
}
