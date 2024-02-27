create table if not exists node (
  inode integer primary key autoincrement not null,
  name text not null,

  kind text check(kind in ('directory', 'file')) not null,

  size bigint not null,
  blocks bigint not null,
  atime timestamp not null,
  mtime timestamp not null,
  ctime timestamp not null,
  crtime timestamp not null,
  perm smallint not null,
  nlink integer not null,
  uid integer not null,
  gid integer not null,
  rdev integer not null,
  flags integer not null,

  parent_inode bigint,

  foreign key (parent_inode) references inode(inode)
);
