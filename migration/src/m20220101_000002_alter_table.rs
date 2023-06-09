// alter table name from `post` to `Item`
// change pice type from `string` to `integer`

use sea_orm_migration::{
    async_trait::async_trait,
    sea_orm::{ConnectionTrait, DeriveMigrationName},
    DbErr, MigrationTrait, SchemaManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        db.execute_unprepared("ALTER TABLE post RENAME TO Item")
            .await?;

        // db.execute_unprepared("ALTER TABLE Item ALTER COLUMN price TYPE INT")
        //     .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `post`")
            .await?;

        Ok(())
    }
}
