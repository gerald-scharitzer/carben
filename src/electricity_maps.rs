pub struct Zone {
	pub name: String,
	pub country: String
}

impl Zone {
	fn new(name: &str, country: &str) -> Zone {
		Zone {
			name: name.to_string(),
			country: country.to_string()
		}
	}
}

pub fn zone() -> Zone {
	Zone::new("Copenhagen", "Denmark")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_zone() {
		const NAME: &str = "Copenhagen";
		const COUNTRY: &str = "Denmark";
		let result = Zone::new(NAME, COUNTRY);
		assert_eq!(result.name, NAME);
		assert_eq!(result.country, COUNTRY);
	}
}
