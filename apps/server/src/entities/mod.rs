//! SeaORM 实体：与 `crates/migration` 中的表结构一一对应。
//!
//! 约定：
//! - 主键统一是应用层生成的雪花 ID（`i64`），因此 `auto_increment = false`；
//! - 时间字段统一 `DateTimeWithTimeZone`；
//! - 软删除字段（`documents.deleted_at`）用 `Option<DateTimeWithTimeZone>` 表示。

pub mod document;
pub mod document_version;
pub mod login_challenge;
pub mod refresh_token;
pub mod space;
pub mod space_member;
pub mod user;

pub use document::Entity as Document;
pub use document_version::Entity as DocumentVersion;
pub use login_challenge::Entity as LoginChallenge;
pub use refresh_token::Entity as RefreshToken;
pub use space::Entity as Space;
pub use space_member::Entity as SpaceMember;
pub use user::Entity as User;
