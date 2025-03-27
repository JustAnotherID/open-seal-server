pub use sea_orm_migration::prelude::*;

mod m20241002_132012_file_info;
mod m20241009_155634_extension;
mod m20241009_160122_extension_tag;
mod m20241009_164900_extension_tag_relation;
mod m20250313_120014_dice_info;
mod m20250313_120024_dice_endpoint;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241002_132012_file_info::Migration),
            Box::new(m20241009_155634_extension::Migration),
            Box::new(m20241009_160122_extension_tag::Migration),
            Box::new(m20241009_164900_extension_tag_relation::Migration),
            Box::new(m20250313_120014_dice_info::Migration),
            Box::new(m20250313_120024_dice_endpoint::Migration),
        ]
    }
}
