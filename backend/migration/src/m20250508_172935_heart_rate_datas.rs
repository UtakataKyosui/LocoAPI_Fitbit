use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "heart_rate_datas",
            &[
            
            ("id", ColType::PkAuto),
            
            ("user_id", ColType::UuidNull),
            ("date", ColType::DateNull),
            ("resting_heart_rate", ColType::IntegerUniq),
            ("fat_burn_minutes", ColType::IntegerNull),
            ("cardio_minutes", ColType::IntegerNull),
            ("peak_minutes", ColType::IntegerNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "heart_rate_datas").await
    }
}
