use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    // 雪花 ID，由应用层生成
                    .col(ColumnDef::new(Users::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Users::Email).string_len(255).not_null().unique_key())
                    // argon2 编码串
                    .col(ColumnDef::new(Users::PasswordHash).string_len(255).not_null())
                    .col(ColumnDef::new(Users::Name).string_len(32).not_null())
                    .col(
                        ColumnDef::new(Users::AvatarUrl)
                            .string_len(512)
                            .not_null()
                            .default(""),
                    )
                    // TOTP 密钥，应用层加密存储；未绑定时为 NULL
                    .col(ColumnDef::new(Users::TotpSecret).string_len(255).null())
                    .col(
                        ColumnDef::new(Users::TotpEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::IsDisabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // 修改密码时 +1，使此前签发的 Token 全部失效
                    .col(
                        ColumnDef::new(Users::TokenVersion)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    Name,
    AvatarUrl,
    TotpSecret,
    TotpEnabled,
    IsDisabled,
    TokenVersion,
    CreatedAt,
    UpdatedAt,
}
