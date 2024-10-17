use config::Config;
use querymate::postgres;
use querymate::ai::{AI, Claude};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

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

    let claude = Claude::new(settings.get_string("claude_api_key").unwrap(), db_schema.to_string());
    let response = claude.generate_response("Show me all users who have placed orders totaling more than $1000").await?;
    println!("Generated SQL Query:\n{}", response);

    Ok(())
}
