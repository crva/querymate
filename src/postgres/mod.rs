use sqlx::Row;

#[tokio::main]
pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://postgres:root@localhost/crva";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let rows = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
    )
    .fetch_all(&pool)
    .await?;

    for row in rows {
        let table_name: &str = row.get("table_name");
        println!("{}", table_name);
    }

    Ok(())
}
