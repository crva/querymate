use config::Config;
use querymate::postgres;
use std::collections::HashMap;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    postgres::connect().unwrap();
}
