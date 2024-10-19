use super::PgResult;
use crate::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

impl User {
    pub async fn new(client: &Client, username: String, password: String) -> PgResult<Uuid> {
        let query = "INSERT INTO users (username, password) VALUES ($1, $2, $3) RETURNING id";
        let row = client.query_one(query, &[&username, &password]).await?;

        let id: Uuid = row.try_get("id")?;
        Ok(id)
    }

    pub async fn get(client: &Client, username: String, password: String) -> PgResult<Uuid> {
        let query = "SELECT id FROM Users WHERE name = $1 AND password = $2";
        let row = client.query_one(query, &[&username, &password]).await?;

        let id: Uuid = row.try_get("id")?;
        Ok(id)
    }
}

impl TryFrom<Row> for User {
    type Error = tokio_postgres::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            password: row.try_get("password")?,
        })
    }
}
