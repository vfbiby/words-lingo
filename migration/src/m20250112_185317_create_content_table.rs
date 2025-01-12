use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Content::Table)
                    .if_not_exists()
                    .col(pk_auto(Content::ContentId))
                    .col(string(Content::Title))
                    .col(
                        ColumnDef::new(Content::ContentType)
                            .enumeration(Content::ContentType, vec![
                                Alias::new("video"),
                                Alias::new("article"),
                            ])
                            .not_null(),
                    )
                    .col(text(Content::Content))
                    .col(
                        timestamp_with_time_zone(Content::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Content::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Content {
    Table,
    ContentId,
    Title,
    ContentType,
    Content,
    CreatedAt,
}
