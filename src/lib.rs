use std::env;
use std::io::{Stdout, Write};
use std::result::Result;

mod electricity_maps;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben [version | zones]
	version: print the version
	zones: print the zone name and country";

pub struct MainRunner {
	args: Vec<String>,
	stdout: Stdout
}

impl MainRunner {
	pub fn new() -> MainRunner {
		MainRunner {
			args: env::args().collect(),
			stdout: std::io::stdout()
		}
	}

	pub fn run(&mut self) -> Result<(), String> { // TODO Result<(), Box<dyn Error>>
		let argc = self.args.len();
		if argc == 1 {
			if let Err(err) = writeln!(self.stdout, "{USAGE}") {
				return Err(err.to_string());
			}
		}

		if argc > 1 {
			let command = &self.args[1];
			match command.as_str() {
				"version" => {
					let version = version();
					if let Err(err) = writeln!(self.stdout, "version {version}") {
						return Err(err.to_string());
					}
				}
				"zones" => {
					let zone = electricity_maps::zone();
					let name = zone.name;
					let country = zone.country;
					if let Err(err) = writeln!(self.stdout, "zone {name} {country}") {
						return Err(err.to_string());
					}
				}
				_ => {
					if let Err(err) = writeln!(self.stdout, "{USAGE}") {
						return Err(err.to_string());
					}
				}
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_version() {
		let result = version();
		assert_eq!(result, 1);
	}
}
