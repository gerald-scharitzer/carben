//! # Carben
//! optimizes the greenhouse gas emissions of computing workloads.

use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::result::Result;

use config::Config;
use format::Format;

mod config;
pub mod electricity_maps;
mod format;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben [options]

No option prints this usage message. Multiple options are separated by spaces.

Options:

	config file
		read the configuration from the file with the given name

	f
		short for \"format\"

	format csv | json
		print the output in the specified format

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
	stdout: Box<dyn Write>,
	stderr: Box<dyn Write>
}

impl MainRunner {
	pub fn new() -> MainRunner {
		MainRunner {
			args: env::args().collect(),
			stdout: Box::new(io::stdout()),
			stderr: Box::new(io::stderr())
		}
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let mut config = Config::new(&self.args)?;
		let mut args = self.args.iter();
		args.next(); // skip the program name
		let mut has_args = false;
		let mut argx = 0; // argument index

		while let Some(arg) = args.next() {
			has_args = true;
			argx += 1;
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
				"f" | "format" => {
					let subarg = args.next();
					match subarg {
						Some(subarg) => {
							match subarg.as_str() {
								"csv" => config.format = Format::Csv,
								"json" => config.format = Format::Json,
								_ => writeln!(self.stdout, "{USAGE}")?
							}
						},
						None => writeln!(self.stdout, "{USAGE}")?
					}
				}
				"health" => { // TODO move to provider
					let health = electricity_maps::health::health()?;
					health.result()?;
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
					match config.format {
						Format::Csv => {
							let zones = electricity_maps::zone::zones_csv()?;
							writeln!(self.stdout, "{zones}")?;
						}
						Format::Json => {
							let zones = electricity_maps::zone::zones_json()?;
							writeln!(self.stdout, "{zones}")?;
						}
					}
				}
				"--" => {
					// TODO iterate through the provider options
					writeln!(self.stderr, "warning: option \"--\" is not implemented")?;
				}
				_ => {
					writeln!(self.stderr, "argument at index {argx} is not implemented")?;
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
