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

pub fn zone() -> Zone {
	Zone::new("Copenhagen".to_string(), Some("Denmark".to_string()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_zone() {
		const NAME: &str = "Copenhagen";
		const COUNTRY: &str = "Denmark";
		let result = Zone::new(NAME.to_string(), Some(COUNTRY.to_string()));
		assert_eq!(result.name, NAME);
		assert_eq!(result.country, Some(COUNTRY.to_string()));
	}
}
