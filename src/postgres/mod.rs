use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:root@localhost/crva";
    let mut conn = sqlx::postgres::PgPool::connect(url).await?;

    println!("Connected!");
    Ok(())
}
