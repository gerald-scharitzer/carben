use std::error::Error;
use std::fs;
use std::io::{stdin, Read};

use super::format::Format;

const DEFAULT_PROVIDER: &str = "electricity_maps";
const DEFAULT_FORMAT: Format = Format::Json;

pub struct Config {
	pub provider: String,
	pub format: Format
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> { // TODO put args in separate module
		Ok(Config { provider: DEFAULT_PROVIDER.to_string(), format: DEFAULT_FORMAT })
	}
}

pub fn from_path(path: &str) -> Result<String, Box<dyn Error>> {
	if path == "-" {
		let mut config = String::new();
		stdin().read_to_string(&mut config)?; // TODO get from MainRunner
		Ok(config)
	} else {
		let config = fs::read_to_string(path)?;
		Ok(config)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let default_config = Config::new(&["carben".to_string()]);
		match default_config {
			Ok(config) => assert_eq!(config.provider, DEFAULT_PROVIDER),
			Err(e) => panic!("Error: {}", e)
		}
	}		
		
	#[test]
	fn test_from_path() {
		let config_result = from_path("stdin.yaml");
		match config_result {
			Ok(config) => assert_eq!(config, "key: value\n"),
			Err(e) => panic!("Error: {}", e)
		}
	}
}
