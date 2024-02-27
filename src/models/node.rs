use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use fuse::{FileAttr, FileType};

pub use crate::schema::NodeKind;
use crate::utils::datetime_into_timespec::datetime_into_timespec;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::node)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Node {
    pub inode: i64,
    pub parent_inode: Option<i64>,

    pub name: String,

    pub kind: NodeKind,

    pub size: i64,
    pub blocks: i64,
    pub atime: NaiveDateTime,
    pub mtime: NaiveDateTime,
    pub ctime: NaiveDateTime,
    pub crtime: NaiveDateTime,
    pub perm: i16,
    pub nlink: i32,
    pub uid: i32,
    pub gid: i32,
    pub rdev: i32,
    pub flags: i32,
}

impl Into<FileAttr> for Node {
    fn into(self) -> FileAttr {
        FileAttr {
            ino: self.inode as u64,
            size: self.size as u64,
            blocks: self.blocks as u64,
            atime: datetime_into_timespec(self.atime),
            mtime: datetime_into_timespec(self.mtime),
            ctime: datetime_into_timespec(self.ctime),
            crtime: datetime_into_timespec(self.crtime),
            kind: self.get_fuse_kind(),
            perm: self.perm as u16,
            nlink: self.nlink as u32,
            uid: self.uid as u32,
            gid: self.gid as u32,
            rdev: self.rdev as u32,
            flags: self.flags as u32,
        }
    }
}

impl Node {
    pub fn get_fuse_kind(&self) -> FileType {
        match self.kind {
            NodeKind::File => FileType::RegularFile,
            NodeKind::Directory => FileType::Directory,
        }
    }
}
