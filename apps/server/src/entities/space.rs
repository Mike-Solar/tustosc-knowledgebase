//! `spaces`：空间（知识库）表。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "spaces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    /// 描述，未填写时为空串。
    pub description: String,
    /// 创建者（所有者）用户 ID。
    pub owner_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    Owner,
    #[sea_orm(has_many = "super::space_member::Entity")]
    Members,
    #[sea_orm(has_many = "super::document::Entity")]
    Documents,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<super::space_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Documents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
