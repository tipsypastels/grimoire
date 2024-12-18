use futures::stream::BoxStream;
use sqlx::{migrate, query, query_as, sqlite::SqlitePoolOptions, Result, SqlitePool};

mod types;

pub use types::*;

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

    pub async fn get_node(&self, id: i64) -> Result<Option<DbNode>> {
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
        query_as(r#"SELECT * FROM nodes ORDER BY name ASC"#).fetch(&self.pool)
    }

    pub async fn insert_node(&self, node: DbNewNode<'_>) -> Result<i64> {
        let res = query(r#"INSERT INTO nodes (path, name, kind, text) VALUES (?1, ?2, ?3, ?4)"#)
            .bind(node.path)
            .bind(node.name)
            .bind(node.kind)
            .bind(node.text)
            .execute(&self.pool)
            .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn insert_node_dependency(&self, from: i64, to: i64) -> Result<i64> {
        let res = query(r#"INSERT INTO node_dependencies (from_id, to_id) VALUES (?1, ?2)"#)
            .bind(from)
            .bind(to)
            .execute(&self.pool)
            .await?;
        Ok(res.last_insert_rowid())
    }
}
