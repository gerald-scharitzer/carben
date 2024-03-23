use std::error::Error;
use std::fs;
use std::io::{stdin, Read};

pub fn config(filename: &str) -> Result<String, Box<dyn Error>> {
    if filename == "-" {
        let mut config = String::new();
        stdin().read_to_string(&mut config)?; // TODO get from MainRunner
        Ok(config)
    } else {
        let config = fs::read_to_string(filename)?;
        Ok(config)
    }
}
