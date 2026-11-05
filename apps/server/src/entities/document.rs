//! `documents`：文档表（Markdown 内容 + 乐观锁版本号 + 软删除）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 所属空间。
    pub space_id: i64,
    /// 父文档 ID；v1 全部为 `None`（空间根目录平铺），v2 目录树预留。
    pub parent_id: Option<i64>,
    /// 同一空间内活跃文档标题唯一（回收站中的文档不占位）。
    pub title: String,
    /// 内容格式，当前固定为 `markdown`。
    pub format: String,
    /// 内容版本号，每次内容变更 +1，用作乐观锁比较对象。
    pub revision: i64,
    /// 当前全量 Markdown 内容。
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub created_by: i64,
    pub updated_by: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    /// 软删除时间；非 `None` 表示在回收站（保留 30 天）。
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::space::Entity",
        from = "Column::SpaceId",
        to = "super::space::Column::Id"
    )]
    Space,
    /// 自关联：父文档（v2 目录树）。
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    Parent,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id"
    )]
    CreatedBy,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UpdatedBy",
        to = "super::user::Column::Id"
    )]
    UpdatedBy,
    #[sea_orm(has_many = "super::document_version::Entity")]
    Versions,
}

impl Related<super::space::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Space.def()
    }
}

impl Related<super::document_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Versions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
