use sea_orm_migration::prelude::*;

use crate::m20261008_000001_create_users::Users;
use crate::m20261008_000006_create_documents::Documents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DocumentVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentVersions::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DocumentVersions::DocId).big_integer().not_null())
                    // 与 documents.revision 对应；回滚也生成新版本号，不覆盖历史
                    .col(ColumnDef::new(DocumentVersions::Version).big_integer().not_null())
                    .col(ColumnDef::new(DocumentVersions::Title).string_len(255).not_null())
                    .col(ColumnDef::new(DocumentVersions::Content).text().not_null())
                    // 保存备注，可选
                    .col(ColumnDef::new(DocumentVersions::Note).string_len(255).null())
                    .col(
                        ColumnDef::new(DocumentVersions::SizeBytes)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DocumentVersions::SavedBy).big_integer().not_null())
                    .col(
                        ColumnDef::new(DocumentVersions::SavedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_document_versions_doc_id")
                            .from(DocumentVersions::Table, DocumentVersions::DocId)
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_document_versions_saved_by")
                            .from(DocumentVersions::Table, DocumentVersions::SavedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uniq_document_versions_doc_version")
                    .table(DocumentVersions::Table)
                    .col(DocumentVersions::DocId)
                    .col(DocumentVersions::Version)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(DocumentVersions::Table).to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum DocumentVersions {
    Table,
    Id,
    DocId,
    Version,
    Title,
    Content,
    Note,
    SizeBytes,
    SavedBy,
    SavedAt,
}
