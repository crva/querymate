use config::Config;
use querymate::postgres;

#[tokio::main]
async fn main() {
    connect_to_postgres().await.unwrap()
}

async fn connect_to_postgres() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

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
