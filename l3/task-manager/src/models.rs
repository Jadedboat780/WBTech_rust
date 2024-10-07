use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgQueryResult};

pub type PgResult<T> = Result<T, sqlx::Error>;

#[derive(Serialize, Deserialize)]
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
        let task = sqlx::query_as!(Task, "INSERT INTO tasks (title) VALUES ($1) RETURNING *", new_task.title)
            .fetch_one(pool)
            .await?;
        Ok(task)
    }

    pub async fn get(pool: &PgPool, id: i32) -> PgResult<Task> {
        sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn complete(pool: &PgPool, id: i32) ->PgResult<PgQueryResult> {
        sqlx::query!("UPDATE tasks SET completed = $1 WHERE id = $2", true, id)
            .execute(pool)
            .await
    }
}
