use std::error::Error;

use carben::electricity_maps::health;

#[test]
fn test_get() -> Result<(), Box<dyn Error>>{
	let health = health::get()?;
	assert!(health.result().is_ok());
    Ok(())
}
