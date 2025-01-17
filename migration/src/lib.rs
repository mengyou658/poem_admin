pub use sea_schema::migration::*;

pub mod db_utils;
mod migrations;

pub use migrations::*;

pub struct Migrator;
pub static DATA_DIR: &str = "migration/data/";

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
