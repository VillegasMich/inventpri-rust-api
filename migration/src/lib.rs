pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_alter_table;
mod m20220101_000003_alter_pirce_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // add all the migrations
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_alter_table::Migration),
            // Box::new(m20220101_000003_alter_pirce_type::Migration),
            // sea dosn't allow to change the type, we can store it as varchar while we are not using it for operations
        ]
    }
}
