use std::error::Error;

use carben;

fn main() -> Result<(), Box<dyn Error>> { // TODO error shall result in exit code 1
	carben::MainRunner::new().run()
}
