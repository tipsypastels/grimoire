use camino::{Utf8Path, Utf8PathBuf};
use sqlx::{error::BoxDynError, prelude::FromRow, Database, Decode, Result, Sqlite, Type};

#[derive(FromRow)]
pub struct DbNode {
    pub id: i32,
    pub path: DbNodePath,
    pub name: Box<str>,
    pub kind: Box<str>,
    pub text: Box<str>,
}

pub struct DbNodePath(pub Box<Utf8Path>);

impl Type<Sqlite> for DbNodePath {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for DbNodePath {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let path = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self(Box::from(Utf8PathBuf::from(path))))
    }
}

pub struct DbNewNode<'a> {
    pub path: &'a str,
    pub name: &'a str,
    pub kind: &'a str,
    pub text: &'a str,
}

impl<'a> From<crate::node::NewNode<'a>> for DbNewNode<'a> {
    fn from(node: crate::node::NewNode<'a>) -> Self {
        Self {
            path: node.path.rel().as_str(),
            name: node.data.name(),
            kind: node.data.kind().as_str(),
            text: node.data.text(),
        }
    }
}

#[derive(FromRow)]
pub struct DbNodeTag {
    pub id: i32,
    pub node_id: i32,
    pub tag: Box<str>,
}

#[derive(FromRow)]
pub struct DbNodeReference {
    pub id: i32,
    pub referrer_id: i32,
    pub referrent_id: i32,
}
