//! `users`：用户账号表（邮箱 + 密码 + TOTP 两步验证）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// 雪花 ID，由应用层生成。
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 登录邮箱，全局唯一。
    pub email: String,
    /// argon2 编码后的密码散列。
    pub password_hash: String,
    /// 昵称（最长 32 字符）。
    pub name: String,
    /// 头像地址，未设置时为空串。
    pub avatar_url: String,
    /// TOTP 密钥（应用层加密存储），未绑定两步验证时为 `None`。
    pub totp_secret: Option<String>,
    /// 是否已启用 TOTP 两步验证。
    pub totp_enabled: bool,
    /// 是否被禁用（封禁）。
    pub is_disabled: bool,
    /// 修改密码时 +1，使此前签发的 Token 全部失效。
    pub token_version: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// 用户拥有的刷新令牌。
    #[sea_orm(has_many = "super::refresh_token::Entity")]
    RefreshTokens,
    /// 用户进行中的登录挑战（TOTP）。
    #[sea_orm(has_many = "super::login_challenge::Entity")]
    LoginChallenges,
    /// 用户创建的空间。
    #[sea_orm(has_many = "super::space::Entity")]
    OwnedSpaces,
    /// 用户加入空间的关系记录。
    #[sea_orm(has_many = "super::space_member::Entity")]
    SpaceMemberships,
}

impl Related<super::refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl Related<super::login_challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoginChallenges.def()
    }
}

impl Related<super::space::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OwnedSpaces.def()
    }
}

impl Related<super::space_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SpaceMemberships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
