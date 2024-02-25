use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_node_table = r#"
            create table if not exists node (
              inode integer primary key autoincrement not null,
              name varchar not null,

              kind varchar not null,

              size bigint not null,
              blocks bigint not null,
              atime text not null,
              mtime text not null,
              ctime text not null,
              crtime text not null,
              perm smallint not null,
              nlink integer not null,
              uid integer not null,
              gid integer not null,
              rdev integer not null,
              flags integer not null,
              blksize integer not null,

              parent_inode bigint,

              foreign key (parent_inode) references inode(inode)
            );
        "#;

        manager
            .get_connection()
            .execute(
                sea_orm::Statement::from_string(
                    manager.get_database_backend(),
                    create_node_table
                )
            )
            .await?;

        Ok(())
    }
async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_node_table = "drop table node";

        manager
            .get_connection()
            .execute(
                sea_orm::Statement::from_string(
                    manager.get_database_backend(),
                    drop_node_table
                )
            )
            .await?;

        Ok(())
    }
}
