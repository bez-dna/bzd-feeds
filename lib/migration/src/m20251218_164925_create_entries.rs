use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Entries;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Entries::Table)
                    .col(uuid(Entries::EntryId).primary_key())
                    .col(uuid(Entries::FeedId))
                    .col(uuid(Entries::MessageId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("entries_feed_id_message_id_udx")
                    .unique()
                    .table(Entries::Table)
                    .col(Entries::MessageId)
                    .col(Entries::FeedId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entries::Table).to_owned())
            .await
    }
}
