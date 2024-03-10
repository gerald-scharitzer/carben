use std::error::Error;

use carben;

pub fn main() -> Result<(), Box<dyn Error>> {
	let mut runner = carben::MainRunner::new();
	runner.run()?;
	Ok(())
}
