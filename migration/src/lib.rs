#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_roles;
mod m20250101_000002_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_roles::Migration),
            Box::new(m20250101_000002_create_users::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
