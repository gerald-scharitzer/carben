use std::env;
use std::error::Error;
use std::io::{self, Stdout, Write};
use std::result::Result;

use config::Config;

mod config;
mod electricity_maps;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben [config file | health | version | zones]
	config file: read the configuration from the file with the given name
	health: print the API health
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
			stdout: io::stdout()
		}
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let config = Config::new(&self.args)?;
		let argc = self.args.len();
		if argc == 1 {
			writeln!(self.stdout, "{USAGE}")?;
		}

		if argc > 1 { // TODO iterate through the arguments
			let command = &self.args[1];
			match command.as_str() {
				"config" => {
					if argc > 2 {
						let config_file = &self.args[2];
						let config = config::from_path(config_file)?;
						writeln!(self.stdout, "config {config}")?;
					} else {
						writeln!(self.stdout, "{USAGE}")?;
					}
				}
				"health" => {
					let health = electricity_maps::health();
					let state = health.monitors.state;
					let status = health.status;
					writeln!(self.stdout, "health {state} {status}")?;
				}
				"version" => {
					let version = version();
					writeln!(self.stdout, "version {version}")?;
				}
				"zones" => {
					let zones = electricity_maps::zones();
					for (key, zone) in zones {
						let name = zone.name;
						let country = zone.country;
						match country {
							Some(country) => writeln!(self.stdout, "zone {key} {name} {country}")?,
							None => writeln!(self.stdout, "zone {key} {name}")?
						}
					}
				}
				_ => {
					writeln!(self.stdout, "{USAGE}")?;
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
