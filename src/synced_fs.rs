use crate::models::node::{self, NodeKind};
use std::cmp::max;
use fuse::{FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};
use futures_util::Future;
use grammers_client::Client;
use sea_orm::{entity::prelude::*, query::*};
use sea_orm::{DatabaseConnection, EntityTrait};
use time::Timespec;
use std::ffi::OsStr;
use libc::ENOENT;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

pub struct SyncedFs<'a> {
    db_connection: &'a DatabaseConnection,
    tg_client: &'a Client
}

impl<'a> SyncedFs<'a> {
    pub fn new(db_connection: &'a DatabaseConnection, tg_client: &'a Client) -> Self {
        Self { db_connection, tg_client }
    }
}

// TODO: Move me!
fn wrap_async_call(future: impl Future + Send + 'static) {
    let current_handle = tokio::runtime::Handle::current();
    current_handle.block_on(future);
}

impl<'a> Filesystem for SyncedFs<'a> {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let owned_name = name.to_str().unwrap().to_owned();
        let db_connection = self.db_connection.clone();

        log::info!("Looking up node with parent_inode: {}, name: {}", parent, owned_name);

        wrap_async_call(async move {
            let child_node_predicate = node::Entity::find()
                .filter(
                    Condition::all()
                        .add(node::Column::ParentInode.eq(parent))
                        .add(node::Column::Name.eq(owned_name))
                )
                .one(&db_connection)
                .await
                .unwrap();

            match child_node_predicate {
                Some(child_node) => reply.entry(&TTL, &child_node.into(), 0),
                None => reply.error(ENOENT),
            };
        });
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        log::info!("Getting attributes for node with inode: {}", ino);

        let db_connection = self.db_connection.clone();
        wrap_async_call(async move {
            let node_predicate: Option<node::Model> = node::Entity::find()
                .filter(node::Column::Inode.eq(ino))
                .one(&db_connection)
                .await
                .unwrap();

            println!("{:?}", node_predicate);

            match node_predicate {
                Some(node) => reply.attr(&TTL, &node.into()),
                None => reply.error(ENOENT),
            }
        });
    }

    fn read(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, _size: u32, reply: ReplyData) {
        reply.error(ENOENT);
        todo!();
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        log::info!("Reading directory with inode: {}", ino);

        let db_connection = self.db_connection.clone();
        wrap_async_call(async move {
            let directory_node_predicate: Option<node::Model> = node::Entity::find()
                .filter(
                    Condition::all()
                        .add(node::Column::Inode.eq(ino))
                        .add(node::Column::Kind.eq(NodeKind::Directory))
                )
                .one(&db_connection)
                .await
                .unwrap();

            match directory_node_predicate {
                Some(directory_node) => {
                    let directory_entries: Vec<node::Model> = node::Entity::find()
                        .filter(node::Column::ParentInode.eq(ino))
                        .all(&db_connection)
                        .await
                        .unwrap();

                    let parent_inode = match directory_node.parent_inode {
                        Some(inode) => inode,
                        None => directory_node.inode,
                    };

                    let mut entries_reply_data = vec![
                        (directory_node.inode, FileType::Directory, String::from(".")),
                        (parent_inode, FileType::Directory, String::from(".."))
                    ];

                    for entry in directory_entries {
                        entries_reply_data.push(
                            (entry.inode, entry.get_fuse_kind(), entry.name)
                        );
                    }

                    log::info!("Discovered {} entries for inode: {}", entries_reply_data.len(), directory_node.inode);
                    log::info!("Returning with offset: {}", offset);

                    for (index, reply_data) in entries_reply_data.into_iter().enumerate().skip(offset as usize) {
                        println!("{index}: {:?}", reply_data);
                        reply.add(reply_data.0 as u64, (index + 1) as i64, reply_data.1, reply_data.2);
                    }

                    reply.ok();
                },
                None => reply.error(ENOENT),
            }
        })
    }
}
