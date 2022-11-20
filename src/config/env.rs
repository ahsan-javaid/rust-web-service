use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

pub struct Config {
  pub port: String
}

impl Default for Config {
  fn default() -> Config {
    Config {
          port: "8080".to_string(),
      }
  }
}

pub fn load_env() -> Config {
    if let Ok(lines) = read_lines("./.env") {
        for line in lines {
            if let Ok(val) = line {
              let v: Vec<&str> = val.split("=").collect();
              env::set_var(v[0], v[1]);
            }
        }
    }
    // Get env variables and set default values
    let mut c = Config::default();
    
    match env::var("PORT") {
      Ok(v) => c.port = v,
      Err(_) => println!("port env variable not found")
   }
   
   c
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
