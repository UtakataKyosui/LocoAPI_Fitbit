#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250508_172935_heart_rate_datas;
mod m20250508_173546_sleep_datas;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250508_172935_heart_rate_datas::Migration),
            Box::new(m20250508_173546_sleep_datas::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}