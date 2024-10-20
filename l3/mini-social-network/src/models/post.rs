use super::PgResult;
use crate::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub likes: i32,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub login_user: super::user::LoginUser,
    pub content: String,
}

impl Post {
    pub async fn new(client: &Client, user_id: Uuid, content: String) -> PgResult<Uuid> {
        let query = "INSERT INTO posts (user_id, content, likes) VALUES ($1, $2, $3) RETURNING id";
        let row = client.query_one(query, &[&user_id, &content, &0]).await?;

        let id: Uuid = row.try_get("id")?;
        Ok(id)
    }

    pub async fn get(client: &Client, post_id: Uuid) -> PgResult<Self> {
        let query = "SELECT id, user_id, content, likes FROM posts WHERE id = $1";
        let row = client.query_one(query, &[&post_id]).await?;

        let post = Post::try_from(row)?;
        Ok(post)
    }

    pub async fn delete(client: &Client, post_id: Uuid) -> PgResult<()> {
        let query = "DELETE FROM Posts WHERE id = $1";
        client.execute(query, &[&post_id]).await?;
        Ok(())
    }

    pub async fn add_like(client: &Client, post_id: Uuid) -> PgResult<()> {
        let query = "UPDATE posts SET likes = likes + 1 WHERE id = $1";
        client.execute(query, &[&post_id]).await?;
        Ok(())
    }
}

impl TryFrom<Row> for Post {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            content: row.try_get("content")?,
            likes: row.try_get("likes")?,
        })
    }
}
