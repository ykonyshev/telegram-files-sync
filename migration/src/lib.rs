pub use sea_orm_migration::prelude::*;

mod m20240224_192755_create_node_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240224_192755_create_node_table::Migration),
        ]
    }
}
