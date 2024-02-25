use crate::models::node;
use fuse::{Filesystem, ReplyEntry, Request};
use futures::executor;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::ffi::OsStr;
use std::time::Duration;

const TTL: Duration = Duration::from_secs(1);

pub struct SyncedFs<'a> {
    db_connection: &'a DatabaseConnection,
}

impl<'a> SyncedFs<'a> {
    pub fn new(db_connection: &'a DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl<'a> Filesystem for SyncedFs<'a> {
    fn lookup(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEntry) {
        executor::block_on(async {
            let child_nodes: Vec<node::Model> = node::Entity::find()
                .filter(node::Column::ParentInode.eq(_parent))
                .all(self.db_connection)
                .await
                .unwrap();

            for node in child_nodes {
                println!("{:?}", node);
            }
        })
    }
}
