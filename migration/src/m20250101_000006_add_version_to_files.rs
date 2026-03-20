use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Files::Table)
                    .add_column(
                        ColumnDef::new(Files::Version)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Files::Table)
                    .drop_column(Files::Version)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Files {
    Table,
    Version,
}
