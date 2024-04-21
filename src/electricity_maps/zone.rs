use std::collections::HashMap;

use super::web;

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

pub fn zones() -> HashMap<String, Zone> {
	let mut zones = HashMap::new();
	zones.insert("DK011".to_string(), Zone::new("Copenhagen".to_string(), Some("Denmark".to_string())));
	zones
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
		let zones = zones();
		assert_eq!(zones.len(), 1);
		let zone = zones.get(KEY).unwrap();
		assert_eq!(zone.name, NAME);
		assert_eq!(zone.country, Some(COUNTRY.to_string()));
	}
}
