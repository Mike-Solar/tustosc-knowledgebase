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
                    .table(LoginChallenges::Table)
                    .if_not_exists()
                    // 即接口中的 challenge_id，一次性使用
                    .col(
                        ColumnDef::new(LoginChallenges::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LoginChallenges::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoginChallenges::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoginChallenges::ConsumedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(LoginChallenges::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_login_challenges_user_id")
                            .from(LoginChallenges::Table, LoginChallenges::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_login_challenges_user_id")
                    .table(LoginChallenges::Table)
                    .col(LoginChallenges::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoginChallenges::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LoginChallenges {
    Table,
    Id,
    UserId,
    ExpiresAt,
    ConsumedAt,
    CreatedAt,
}
