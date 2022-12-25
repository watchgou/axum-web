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
pub struct Server {
    pub host: String,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    redis: Redis,
    pub server: Server,
}

pub fn configuration() -> Result<Config, ()> {
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
    Ok(config)
}

pub fn reids_connect() -> impl redis::ConnectionLike {
    let config = configuration();

    match config {
        Ok(config) => {
            let redis_conn_url = format!("redis://{}:{}", config.redis.host, config.redis.port);

            redis::Client::open(redis_conn_url)
                .expect("Invalid connection URL")
                .get_connection()
                .expect("failed to connect to Redis")
        }
        Err(_) => redis::Client::open("redis://127.0.0.1:4379")
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis"),
    }
}

pub fn server_configuration() -> ([u8; 4], u16) {
    let config = configuration();

    match config {
        Ok(config) => {
            let v_host: Vec<u8> = config
                .server
                .host
                .split(',')
                .map(|v| match v.parse::<u8>() {
                    Ok(h) => h,
                    Err(_) => 0,
                })
                .collect();
            let v_host = v_host.as_slice();
            let mut host: [u8; 4] = [0, 0, 0, 0];
            for i in 0..4 {
                host[i] = v_host[i];
            }
            let port = config.server.port.parse::<u16>().unwrap();
            (host, port)
        }
        Err(_) => ([0, 0, 0, 0], 9000),
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]

    fn test() {
        reids_connect();
    }
}
