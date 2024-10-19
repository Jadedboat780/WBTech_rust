pub mod post;
pub mod user;

type PgResult<T> = Result<T, tokio_postgres::Error>;
