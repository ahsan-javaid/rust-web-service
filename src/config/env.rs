use std::env;
use std::fs;

pub struct Config {
    pub port: String,
    pub db: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: "8080".to_string(),
            db: "sqlite.db".to_string(),
        }
    }
}

pub fn load_env() -> Config {
    let contents =
        fs::read_to_string("./.env").expect("Please provide .env file in the root of project");

    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines {
        let v: Vec<&str> = line.split("=").collect();
        env::set_var(v[0], v[1]);
    }

    // Get env variables and set default values
    let mut c = Config::default();

    match env::var("PORT") {
        Ok(v) => c.port = v,
        Err(_) => println!("port env variable not found"),
    }

    match env::var("DB") {
        Ok(v) => c.db = v,
        Err(_) => println!("db env variable not found"),
    }

    c
}
