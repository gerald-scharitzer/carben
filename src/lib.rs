use std::env;
use std::io::{Result, Stdout, Write};

mod electricity_maps;

pub fn version() -> i32 {
	1
}

const USAGE: &str = "Usage: carben version";

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

	pub fn run(&mut self) -> Result<()> {
		let argc = self.args.len();
		let result = writeln!(self.stdout, "argc {argc}");
		if result.is_err() {
			return result;
		}

		if argc == 1 {
			return writeln!(self.stdout, "{USAGE}");
		}

		if argc > 1 {
			let command = &self.args[1];
			match command.as_str() {
				"version" => {
					let version = version();
					return writeln!(self.stdout, "version {version}");
				}
				"zones" => {
					let zone = electricity_maps::zone();
					let name = zone.name;
					let country = zone.country;
					return writeln!(self.stdout, "zone {name} {country}");
				}
				_ => {
					return writeln!(self.stdout, "{USAGE}");
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
