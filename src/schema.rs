#[derive(Debug, diesel_derive_enum::DbEnum)]
pub enum NodeKind {
    Directory,
    File,
}

diesel::table! {
    use super::NodeKindMapping;
    use diesel::sql_types::*;

    node (inode) {
        inode -> BigInt,
        name -> Text,
        kind -> NodeKindMapping,
        size -> BigInt,
        blocks -> BigInt,
        atime -> Timestamp,
        mtime -> Timestamp,
        ctime -> Timestamp,
        crtime -> Timestamp,
        perm -> SmallInt,
        nlink -> Integer,
        uid -> Integer,
        gid -> Integer,
        rdev -> Integer,
        flags -> Integer,
        parent_inode -> Nullable<BigInt>,
    }
}
