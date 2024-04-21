//! Electricity Maps Zones API https://static.electricitymaps.com/api/docs/index.html#zones

use std::collections::HashMap;

use serde::Deserialize;

use super::web;

/// API response for zones
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ApiZone {
	// Equal names usually refer to equal zones
	pub zoneName: String,
	// One country can have multiple zones
	pub countryName: Option<String>
}

/// Same as ApiZone but with different names
pub struct Zone {
	// Equal names usually refer to equal zones
	pub name: String,
	// One country can have multiple zones
	pub country: Option<String>
}

impl Zone {
	fn new(name: String, country: Option<String>) -> Self {
		Zone { name, country }
	}
}

const PATH: &str = "v3/zones";

pub fn zones_yaml() -> Result<String, Box<dyn std::error::Error>> {
	let text = web::text(PATH)?;
	Ok(text)
}

pub fn api_zones() -> Result<HashMap<String, ApiZone>, Box<dyn std::error::Error>> {
	let response = web::get(PATH)?;
	let api_zones: HashMap<String, ApiZone> = response.json()?;
	Ok(api_zones)
}

pub fn zones() -> Result<HashMap<String, Zone>, Box<dyn std::error::Error>> {
	let api_zones: HashMap<String, ApiZone> = api_zones()?;
	let mut zones = HashMap::new();
	for (key, api_zone) in api_zones {
		let zone = Zone::new(api_zone.zoneName, api_zone.countryName);
		zones.insert(key, zone);
	}
	Ok(zones)
}

#[cfg(test)]
mod tests {
	use super::*;

	const KEY: &str = "DK011";
	const NAME: &str = "Copenhagen";
	const COUNTRY: &str = "Denmark";

	#[test]
	fn test_zone() {
		let zone = Zone::new(NAME.to_string(), Some(COUNTRY.to_string()));
		assert_eq!(zone.name, NAME);
		assert_eq!(zone.country, Some(COUNTRY.to_string()));
	}

	#[test]
	fn test_zones() {
		let mut zones = HashMap::new();
		zones.insert("DK011".to_string(), Zone::new("Copenhagen".to_string(), Some("Denmark".to_string())));
		assert_eq!(zones.len(), 1);
		let zone = zones.get(KEY).unwrap();
		assert_eq!(zone.name, NAME);
		assert_eq!(zone.country, Some(COUNTRY.to_string()));
	}
}
