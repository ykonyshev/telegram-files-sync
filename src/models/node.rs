use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum NodeKind {
    #[sea_orm(string_value = "directory")]
    Directory,
    #[sea_orm(string_value = "file")]
    File
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub inode: i64,
    pub parent_inode: Option<i64>,

    pub name: String,

    pub kind: NodeKind,

    pub size: i64,
    pub blocks: i64,
    pub atime: DateTimeUtc,
    pub mtime: DateTimeUtc,
    pub ctime: DateTimeUtc,
    pub crtime: DateTimeUtc,
    pub perm: i16,
    pub nlink: i32,
    pub uid: i32,
    pub gid: i32,
    pub rdev: i32,
    pub flags: hello0,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentInode", to = "Column::Inode")]
    SelfReferencing,
}

pub struct SelfReferencingLink;

impl Linked for SelfReferencingLink {
    type FromEntity = Entity;

    type ToEntity = Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::SelfReferencing.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
