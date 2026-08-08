use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Memos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Memos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Memos::Title).string().not_null())
                    .col(ColumnDef::new(Memos::Description).text().not_null())
                    .col(
                        ColumnDef::new(Memos::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Memos::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Memos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Memos {
    Table,
    Id,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
}
