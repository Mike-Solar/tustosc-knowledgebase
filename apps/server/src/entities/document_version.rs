//! `document_versions`：文档历史版本快照表。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "document_versions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 所属文档。
    pub doc_id: i64,
    /// 快照对应的 `documents.revision`；回滚也生成新版本号，不覆盖历史。
    pub version: i64,
    /// 快照时的标题。
    pub title: String,
    /// 快照时的完整内容。
    #[sea_orm(column_type = "Text")]
    pub content: String,
    /// 保存备注，可选。
    pub note: Option<String>,
    /// 内容字节数，列表展示用。
    pub size_bytes: i64,
    /// 保存人。
    pub saved_by: i64,
    pub saved_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::document::Entity",
        from = "Column::DocId",
        to = "super::document::Column::Id"
    )]
    Document,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SavedBy",
        to = "super::user::Column::Id"
    )]
    SavedBy,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SavedBy.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
