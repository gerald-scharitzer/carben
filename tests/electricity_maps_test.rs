/// integration tests for the Electricity Maps API

use std::error::Error;

use carben::electricity_maps::health;
use carben::electricity_maps::zone;

#[test]
fn test_health() -> Result<(), Box<dyn Error>>{
	let health = health::health()?;
	assert!(health.result().is_ok());
    Ok(())
}

#[test]
fn test_zones() -> Result<(), Box<dyn Error>> {
	let zones = zone::zones_yaml()?;
	println!("{zones}");
	assert!(zones.len() > 0);
	Ok(())
}
