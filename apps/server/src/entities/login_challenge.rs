//! `login_challenges`：TOTP 登录挑战表（一次性使用）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "login_challenges")]
pub struct Model {
    /// 即对外的 `challenge_id`。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i64,
    pub expires_at: DateTimeWithTimeZone,
    /// 已被消费的时间；`None` 表示未使用。
    pub consumed_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
