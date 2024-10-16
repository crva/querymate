use sqlx::{Pool, Postgres, Row};

pub struct PostgresConnection {
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub ip: String,
}

impl PostgresConnection {
    pub fn new(username: String, password: String, dbname: String, ip: String) -> Self {
        PostgresConnection {
            username,
            password,
            dbname,
            ip,
        }
    }
}

pub async fn connect(
    postgres_connection: PostgresConnection,
) -> Result<Pool<Postgres>, sqlx::Error> {
    let url = format!(
        "postgres://{}:{}@{}/{}",
        postgres_connection.username,
        postgres_connection.password,
        postgres_connection.ip,
        postgres_connection.dbname
    );

    let pool: Pool<Postgres> = sqlx::postgres::PgPool::connect(&url).await?;

    Ok(pool)
}

pub async fn get_db_tables(pool: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let table_name: &str = row.get("table_name");
        println!("{}", table_name);
    }

    Ok(())
}
