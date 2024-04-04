//! # Carben
//! optimizes the greenhouse gas emissions of computing workloads.

use std::env;
use std::error::Error;
use std::io::{self, Stderr, Stdout, Write};
use std::result::Result;

use config::Config;

mod config;
mod electricity_maps;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben [options]

No option prints this usage message. Multiple options are separated by spaces.

Options:

	config file
		read the configuration from the file with the given name

	health
		print the API health

	p
		short for \"provider\"

	provider
		print the provider name

	v
		short for \"version\"

	version
		print the version

	zones
		print the zone name and country

	--
		separate main options from provider options";

const DEFAULT_PROVIDER: &str = "electricity_maps";

pub fn default_provider() -> String {
	DEFAULT_PROVIDER.to_string()
}

pub struct Runner {
	provider: String
}

impl Runner {
	pub fn new() -> Runner {
		Runner {
			provider: default_provider()
		}
	}

	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		match self.provider.as_str() {
			"electricity_maps" => {
				let health = electricity_maps::health()?;
				let state = health.monitors.state;
				let status = health.status;
				println!("health {state} {status}");
			}
			_ => {
				println!("{USAGE}");
			}
		}
		Ok(())
	}
}

/// Run an instance of this application like a main function.
/// - list of arguments
/// - standard output
/// - standard error
pub struct MainRunner {
	args: Vec<String>,
	stdout: Stdout,
	stderr: Stderr
}

impl MainRunner {
	pub fn new() -> MainRunner {
		MainRunner {
			args: env::args().collect(),
			stdout: io::stdout(),
			stderr: io::stderr()
		}
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let config = Config::new(&self.args)?;
		let mut args = self.args.iter();
		args.next(); // skip the program name
		let mut has_args = false;

		while let Some(arg) = args.next() {
			has_args = true;
			match arg.as_str() {
				"config" => {
					let subarg = args.next();
					match subarg {
						Some(subarg) => {
							let config_file = subarg;
							let config = config::from_path(config_file)?;
							writeln!(self.stdout, "config {config}")?;
						},
						None => writeln!(self.stdout, "{USAGE}")?
					}
				}
				"health" => { // TODO move to provider
					let health = electricity_maps::health()?;
					let state = health.monitors.state;
					let status = health.status;
					writeln!(self.stdout, "health {state} {status}")?;
				}
				"p" | "provider" => {
					let provider = &config.provider;
					writeln!(self.stdout, "{provider}")?;
				}
				"v" | "version" => {
					let version = version();
					writeln!(self.stdout, "{version}")?;
				}
				"zones" => { // TODO move to provider
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
				"--" => {
					// TODO iterate through the provider options
					writeln!(self.stderr, "warning: option \"--\" is not implemented")?;
				}
				_ => {
					writeln!(self.stdout, "{USAGE}")?;
				}
			}

		}

		if !has_args {
			writeln!(self.stdout, "{USAGE}")?;
		}

		let runner = Runner::new();
		runner.run()
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
