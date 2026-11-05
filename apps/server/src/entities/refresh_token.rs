//! `refresh_tokens`：刷新令牌表（只存 SHA-256，不存明文）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 所属用户。
    pub user_id: i64,
    /// 令牌的 SHA-256 十六进制摘要，唯一。
    pub token_hash: String,
    /// 签发时的客户端 User-Agent，便于用户管理登录设备。
    pub user_agent: Option<String>,
    pub expires_at: DateTimeWithTimeZone,
    /// 吊销时间；`None` 表示仍然有效。
    pub revoked_at: Option<DateTimeWithTimeZone>,
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
