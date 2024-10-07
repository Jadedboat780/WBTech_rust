use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use tokio::sync::Mutex;

type RedisResult<T> = Result<T, redis::RedisError>;

pub enum TaskNotify {
    CREATED,
    COMPLETED,
}

pub async fn notify_users(
    con: &Mutex<MultiplexedConnection>,
    notify: TaskNotify,
    id: i32
) -> RedisResult<()> {
    let message = match notify {
        TaskNotify::CREATED => format!("Task {} created", id),
        TaskNotify::COMPLETED => format!("Task {} completed", id),
    };

    let mut con = con.lock().await;

    con.publish::<&str, String, ()>("task_notifications", message.clone()).await?;

    con.lpush("task_notifications_list", message).await?;
    con.ltrim("task_notifications_list", 0, 99).await?;

    Ok(())
}