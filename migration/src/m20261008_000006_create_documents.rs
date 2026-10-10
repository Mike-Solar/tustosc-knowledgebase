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
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Documents::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(Documents::SpaceId).big_integer().not_null())
                    // v2 树形目录预留；v1 全部为 NULL（空间根目录平铺）
                    .col(ColumnDef::new(Documents::ParentId).big_integer().null())
                    .col(ColumnDef::new(Documents::Title).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Documents::Format)
                            .string_len(16)
                            .not_null()
                            .default("markdown"),
                    )
                    // 内容版本号，每次内容变更 +1（乐观锁 base_rev 比较对象）
                    .col(
                        ColumnDef::new(Documents::Revision)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    // 当前全量 Markdown 内容
                    .col(
                        ColumnDef::new(Documents::Content)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Documents::CreatedBy).big_integer().not_null())
                    .col(ColumnDef::new(Documents::UpdatedBy).big_integer().not_null())
                    .col(
                        ColumnDef::new(Documents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Documents::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    // 软删除：非空即在回收站，保留 30 天
                    .col(
                        ColumnDef::new(Documents::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_documents_space_id")
                            .from(Documents::Table, Documents::SpaceId)
                            .to(Spaces::Table, Spaces::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_documents_parent_id")
                            .from(Documents::Table, Documents::ParentId)
                            .to(Documents::Table, Documents::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_documents_created_by")
                            .from(Documents::Table, Documents::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_documents_updated_by")
                            .from(Documents::Table, Documents::UpdatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // 空间文档列表（排除回收站）
        manager
            .create_index(
                Index::create()
                    .name("idx_documents_space_id_deleted_at")
                    .table(Documents::Table)
                    .col(Documents::SpaceId)
                    .col(Documents::DeletedAt)
                    .to_owned(),
            )
            .await?;

        // 同空间内活跃文档标题唯一（回收站中的文档不占位），重名返回 409
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX uniq_documents_space_title_active \
                 ON documents (space_id, title) WHERE deleted_at IS NULL",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Documents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Documents {
    Table,
    Id,
    SpaceId,
    ParentId,
    Title,
    Format,
    Revision,
    Content,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
