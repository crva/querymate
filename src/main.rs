use config::Config;
use querymate::ai::{Claude, AI};
use querymate::postgres;

#[tokio::main]
async fn main() {
    let settings: Config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    connect_to_postgres(settings).await.unwrap()
}

async fn request_claude(settings: Config) -> Result<(), Box<dyn std::error::Error>> {
    // postgres::connect().unwrap();
    let db_schema = r#"
    CREATE TABLE users (
        id INT PRIMARY KEY,
        name VARCHAR(100),
        email VARCHAR(100),
        created_at TIMESTAMP
    );

    
    CREATE TABLE orders (
        id INT PRIMARY KEY,
        user_id INT,
        total_amount DECIMAL(10, 2),
        order_date TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

    CREATE TABLE products (
        id INT PRIMARY KEY,
        name VARCHAR(100),
        price DECIMAL(10, 2),
        stock INT
    );

    CREATE TABLE order_items (
        id INT PRIMARY KEY,
        order_id INT,
        product_id INT,
        quantity INT,
        FOREIGN KEY (order_id) REFERENCES orders(id),
        FOREIGN KEY (product_id) REFERENCES products(id)
    );
    "#;

    let claude = Claude::new(
        settings.get_string("claude_api_key").unwrap(),
        db_schema.to_string(),
    );
    let response = claude
        .generate_response("Show me all users who have placed orders totaling more than $1000")
        .await?;

    println!("Generated SQL Query:\n{}", response);

    Ok(())
}

async fn connect_to_postgres(settings: Config) -> Result<(), Box<dyn std::error::Error>> {
    let username = settings
        .clone()
        .get_string("postgres_username")
        .unwrap()
        .to_string();
    let password = settings
        .get_string("postgres_password")
        .unwrap()
        .to_string();
    let dbname = settings.get_string("postgres_dbname").unwrap().to_string();
    let ip = settings.get_string("postgres_ip").unwrap().to_string();

    let postgres_connection = postgres::PostgresConnection {
        username,
        password,
        dbname,
        ip,
    };

    let pool = postgres::connect(postgres_connection).await.unwrap();
    postgres::get_db_tables(&pool).await.unwrap();

    Ok(())
}
