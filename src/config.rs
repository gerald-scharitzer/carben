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

mod test {
	#[test]
	fn test_config() {
		let config_result = super::config("stdin.yaml");
		match config_result {
			Ok(config) => assert_eq!(config, "key: value\n"),
			Err(e) => panic!("Error: {}", e)
		}
	}
}
