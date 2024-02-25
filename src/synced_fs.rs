use std::ffi::OsStr;
use std::time::Duration;
use fuse::{Filesystem, Request, ReplyEntry};
use crate::models::node;
use sea_orm::{EntityTrait, DatabaseConnection};
use libc::ENOENT;

const TTL: Duration = Duration::from_secs(1);

pub struct SyncedFs<'a> {
    db_connection: &'a DatabaseConnection
}

impl<'a> Filesystem for SyncedFs<'a> {
    fn lookup(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEntry) {
    }
}
