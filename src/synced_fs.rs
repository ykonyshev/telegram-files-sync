use crate::models::node::{self, Node, NodeKind};
use crate::schema::node::dsl;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use fuse::{FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};
use grammers_client::Client;
use libc::ENOENT;
use std::cmp::max;
use std::ffi::OsStr;
use time::Timespec;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

pub struct SyncedFs<'a> {
    db_connection: &'a mut SqliteConnection,
    tg_client: &'a Client,
}

impl<'a> SyncedFs<'a> {
    pub fn new(db_connection: &'a mut SqliteConnection, tg_client: &'a Client) -> Self {
        Self {
            db_connection,
            tg_client,
        }
    }
}

impl<'a> Filesystem for SyncedFs<'a> {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let name_as_str = name.to_str().unwrap();

        log::info!(
            "Looking up node with parent_inode: {}, name: {}",
            parent,
            name_as_str
        );

        let child_node_predicate: QueryResult<Node> = dsl::node
            .select(Node::as_select())
            .filter(dsl::parent_inode.eq(parent as i64))
            .filter(dsl::name.eq(name_as_str))
            .first(self.db_connection);

        match child_node_predicate {
            Err(NotFound) => reply.error(ENOENT),
            Err(err) => {
                log::error!("Could not lookup for child of: {parent}, with name: {name_as_str}, due to: {err}");
                reply.error(ENOENT);
            }
            Ok(child_node) => reply.entry(&TTL, &child_node.into(), 0),
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        log::info!("Getting attributes for node with inode: {}", ino);

        let node_predicate: QueryResult<Node> = dsl::node
            .select(Node::as_select())
            .filter(dsl::inode.eq(ino as i64))
            .first(self.db_connection);

        match node_predicate {
            Err(NotFound) => reply.error(ENOENT),
            Err(err) => {
                log::error!("Could not getattr inode: {ino}, due to: {err}");
                reply.error(ENOENT);
            }
            Ok(node) => reply.attr(&TTL, &node.into()),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        reply.error(ENOENT);
        todo!();
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        log::info!("Reading directory with inode: {}", ino);

        let directory_node_predicate: QueryResult<Node> = dsl::node
            .select(Node::as_select())
            .filter(dsl::inode.eq(ino as i64))
            .filter(dsl::kind.eq(NodeKind::Directory))
            .first(self.db_connection);

        match directory_node_predicate {
            Err(NotFound) => reply.error(ENOENT),
            Err(err) => {
                log::error!("Could not readdir inode: {ino}, due to: {err}");
                reply.error(ENOENT);
            }
            Ok(directory_node) => {
                let directory_entries_with_err: QueryResult<Vec<Node>> = dsl::node
                    .select(Node::as_select())
                    .limit(-1)
                    .offset(max(0, offset - 2))
                    .filter(dsl::parent_inode.eq(directory_node.inode))
                    .load(self.db_connection);

                let parent_inode = match directory_node.parent_inode {
                    Some(inode) => inode,
                    None => directory_node.inode,
                };

                let mut entries_reply_data = vec![
                    (directory_node.inode, FileType::Directory, String::from(".")),
                    (parent_inode, FileType::Directory, String::from("..")),
                ];

                match directory_entries_with_err {
                    Err(err) => {
                        log::error!("Could not retrieve entries for directory during readdir, inode: {ino}, due to: {err}");
                    }
                    Ok(directory_entries) => {
                        for entry in directory_entries {
                            entries_reply_data.push((
                                entry.inode,
                                entry.get_fuse_kind(),
                                entry.name,
                            ));
                        }
                    }
                };

                for (index, reply_data) in entries_reply_data
                    .into_iter()
                    .enumerate()
                    .skip(offset as usize)
                {
                    println!("{index}: {:?}", reply_data);
                    reply.add(
                        reply_data.0 as u64,
                        (index + 1) as i64,
                        reply_data.1,
                        reply_data.2,
                    );
                }

                reply.ok();
            }
        };
    }
}
