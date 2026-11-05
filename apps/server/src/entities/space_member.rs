//! `space_members`：空间成员与角色（OWNER / ADMIN / EDITOR / VIEWER）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "space_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub space_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    /// 成员角色：`OWNER` / `ADMIN` / `EDITOR` / `VIEWER`。
    pub role: String,
    pub joined_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::space::Entity",
        from = "Column::SpaceId",
        to = "super::space::Column::Id"
    )]
    Space,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::space::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Space.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
