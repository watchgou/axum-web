use std::{fs::File, io::Read};

use redis;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Redis {
    host: String,
    port: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    redis: Redis,
}

pub fn connect() -> impl redis::ConnectionLike {
    let mut file = match File::open("./application.toml") {
        Ok(f) => f,
        Err(e) => panic!("no such file ./application.toml exception:{}", e),
    };
    let mut toml_str = String::new();

    match file.read_to_string(&mut toml_str) {
        Ok(f) => f,
        Err(e) => panic!("is error {}", e),
    };
    let config: Config = toml::from_str(&toml_str.as_str()).unwrap();

    let redis_conn_url = format!("redis://{}:{}", config.redis.host, config.redis.port);

    let conn = redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");
    conn
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]

    fn test() {
        connect();
    }
}

