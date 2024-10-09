use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgQueryResult};

pub type PgResult<T> = Result<T, sqlx::Error>;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
}

impl Task {
    pub async fn create(pool: &PgPool, new_task: NewTask) -> PgResult<Task> {
        let task = sqlx::query_as::<_, Task>("INSERT INTO tasks (title) VALUES ($1) RETURNING *")
            .bind(new_task.title)
            .fetch_one(pool)
            .await?;
        Ok(task)
    }

    pub async fn get(pool: &PgPool, id: i32) -> PgResult<Task> {
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn complete(pool: &PgPool, id: i32) ->PgResult<PgQueryResult> {
        sqlx::query("UPDATE tasks SET completed = $1 WHERE id = $2")
            .bind(true)
            .bind(id)
            .execute(pool)
            .await
    }
}
