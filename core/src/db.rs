use crate::node::NodePathRel;
use futures::{stream::BoxStream, Stream};
use sqlx::{
    migrate, prelude::FromRow, query, query_as, sqlite::SqlitePoolOptions, Result, SqlitePool,
};

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect("sqlite::memory:")
            .await?;

        migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn get_node(&self, id: i32) -> Result<Option<DbNode>> {
        let node = query_as(r#"SELECT * FROM nodes WHERE id = ?1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(node)
    }

    pub async fn get_node_by_path(&self, path: &str) -> Result<Option<DbNode>> {
        let node = query_as(r#"SELECT * FROM nodes WHERE path = ?1"#)
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;
        Ok(node)
    }

    pub fn get_nodes(&self) -> BoxStream<Result<DbNode>> {
        query_as(r#"SELECT * FROM nodes"#).fetch(&self.pool)
    }

    pub async fn insert_node(&self, node: DbNewNode<'_>) -> Result<()> {
        query(r#"INSERT INTO nodes (path, name, kind, text) VALUES (?1, ?2, ?3, ?4)"#)
            .bind(node.path)
            .bind(node.name)
            .bind(node.kind)
            .bind(node.text)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_node_reference(&self, referrer: i32, referrent: i32) -> Result<()> {
        query(r#"INSERT INTO node_references (referrer_id, referrent_id) VALUES (?1, ?2)"#)
            .bind(referrer)
            .bind(referrent)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(FromRow)]
pub struct DbNode {
    pub id: i32,
    pub path: NodePathRel,
    pub name: Box<str>,
    pub kind: Box<str>,
    pub text: Box<str>,
}

pub struct DbNewNode<'a> {
    pub path: &'a NodePathRel,
    pub name: &'a str,
    pub kind: &'a str,
    pub text: &'a str,
}

impl<'a> From<crate::node::NewNode<'a>> for DbNewNode<'a> {
    fn from(node: crate::node::NewNode<'a>) -> Self {
        Self {
            path: node.path.rel(),
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
