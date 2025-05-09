use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "sleep_datas",
            &[
            
            ("id", ColType::PkAuto),
            
            ("user_id", ColType::UuidNull),
            ("date", ColType::DateNull),
            ("total_minutes_asleep", ColType::IntegerNull),
            ("total_sleep_records", ColType::IntegerNull),
            ("stages", ColType::JsonNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "sleep_datas").await
    }
}
