use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "INSERT INTO roles (id, name, attributes) VALUES (DEFAULT, 'admin', '[]') ON CONFLICT (id) DO NOTHING;
             INSERT INTO roles (id, name, attributes) VALUES (DEFAULT, 'user', '[]') ON CONFLICT (id) DO NOTHING;",
        )
        .await?;

        // admin/admin123
        db.execute_unprepared(
            "INSERT INTO users (id, username, login, password, role_id)
             VALUES (DEFAULT, 'admin', 'admin', '$2b$12$NQaFsIoES8kVJOBa9KoZFeUmyMBDwpAaUI9zKWkGGD/XvWJRxMjMW', 1)
             ON CONFLICT (id) DO NOTHING;",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DELETE FROM users WHERE id = 1")
            .await?;

        db.execute_unprepared("DELETE FROM roles WHERE id = 1")
            .await?;

        Ok(())
    }
}
