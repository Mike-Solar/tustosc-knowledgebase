pub use sea_orm_migration::prelude::*;

mod m20261008_000001_create_users;
mod m20261008_000002_create_refresh_tokens;
mod m20261008_000003_create_login_challenges;
mod m20261008_000004_create_spaces;
mod m20261008_000005_create_space_members;
mod m20261008_000006_create_documents;
mod m20261008_000007_create_document_versions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261008_000001_create_users::Migration),
            Box::new(m20261008_000002_create_refresh_tokens::Migration),
            Box::new(m20261008_000003_create_login_challenges::Migration),
            Box::new(m20261008_000004_create_spaces::Migration),
            Box::new(m20261008_000005_create_space_members::Migration),
            Box::new(m20261008_000006_create_documents::Migration),
            Box::new(m20261008_000007_create_document_versions::Migration),
        ]
    }
}
