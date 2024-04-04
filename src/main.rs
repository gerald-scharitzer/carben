use std::error::Error;

use carben;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { // TODO error shall result in exit code 1
	let mut runner = carben::MainRunner::new();
	runner.run()?;
	Ok(())
}
