use sea_orm_migration::prelude::*;

use crate::m20261008_000001_create_users::Users;
use crate::m20261008_000004_create_spaces::Spaces;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SpaceMembers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SpaceMembers::SpaceId).big_integer().not_null())
                    .col(ColumnDef::new(SpaceMembers::UserId).big_integer().not_null())
                    .col(ColumnDef::new(SpaceMembers::Role).string_len(16).not_null())
                    .col(
                        ColumnDef::new(SpaceMembers::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(SpaceMembers::SpaceId)
                            .col(SpaceMembers::UserId),
                    )
                    .check(Expr::col(SpaceMembers::Role).is_in(["OWNER", "ADMIN", "EDITOR", "VIEWER"]))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_space_members_space_id")
                            .from(SpaceMembers::Table, SpaceMembers::SpaceId)
                            .to(Spaces::Table, Spaces::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_space_members_user_id")
                            .from(SpaceMembers::Table, SpaceMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 「我加入的空间」列表查询
        manager
            .create_index(
                Index::create()
                    .name("idx_space_members_user_id")
                    .table(SpaceMembers::Table)
                    .col(SpaceMembers::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SpaceMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SpaceMembers {
    Table,
    SpaceId,
    UserId,
    Role,
    JoinedAt,
}
