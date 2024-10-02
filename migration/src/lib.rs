pub use sea_orm_migration::prelude::*;

mod m20241002_132012_file_info;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241002_132012_file_info::Migration)]
    }
}
