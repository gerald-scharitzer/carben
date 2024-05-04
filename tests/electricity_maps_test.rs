//! integration tests for the Electricity Maps API

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
fn test_zones_json() -> Result<(), Box<dyn Error>> {
	let zones = zone::zones_json()?;
	println!("{zones}");
	assert!(zones.len() > 0);
	Ok(())
}

#[test]
fn test_zones() -> Result<(), Box<dyn Error>> {
	let zones = zone::zones()?;
	assert!(zones.len() > 0);
	for (key, zone) in zones {
		assert!(key.len() > 0);
		assert!(zone.name.len() > 0);
	}
	Ok(())
}

#[test]
fn test_zones_csv() -> Result<(), Box<dyn Error>> {
	let csv = zone::zones_csv()?;
	println!("{csv}");
	assert!(csv.len() > 0);
	Ok(())
}
