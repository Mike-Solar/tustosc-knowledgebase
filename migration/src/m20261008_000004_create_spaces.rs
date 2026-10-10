use sea_orm_migration::prelude::*;

use crate::m20261008_000001_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Spaces::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Spaces::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Spaces::Name).string_len(128).not_null())
                    .col(
                        ColumnDef::new(Spaces::Description)
                            .string_len(1024)
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Spaces::OwnerId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Spaces::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Spaces::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_spaces_owner_id")
                            .from(Spaces::Table, Spaces::OwnerId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Spaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Spaces {
    Table,
    Id,
    Name,
    Description,
    OwnerId,
    CreatedAt,
    UpdatedAt,
}
