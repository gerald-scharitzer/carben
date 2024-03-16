use std::env;
use std::error::Error;
use std::io::{Stdout, Write};
use std::result::Result;

mod electricity_maps;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben [health | version | zones]
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
			stdout: std::io::stdout()
		}
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let argc = self.args.len();
		if argc == 1 {
			writeln!(self.stdout, "{USAGE}")?;
		}

		if argc > 1 {
			let command = &self.args[1];
			match command.as_str() {
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
					let zone = electricity_maps::zone();
					let name = zone.name;
					let country = zone.country;
					match country {
						Some(country) => writeln!(self.stdout, "zone {name} {country}")?,
						None => writeln!(self.stdout, "zone {name}")?
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
