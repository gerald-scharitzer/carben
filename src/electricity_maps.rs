pub struct Zone<'a> {
	// Equal names usually refer to equal zones, so own the name
	pub name: String,
	// One country can have multiple zones, so borrow the country
	pub country: &'a str
}

impl<'a> Zone<'a> {
	fn new(name: String, country: &'a str) -> Self {
		Zone {
			name: name,
			country: country
		}
	}
}

pub fn zone<'a>() -> Zone<'a> {
	Zone::new("Copenhagen".to_string(), "Denmark")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_zone() {
		const NAME: &str = "Copenhagen";
		const COUNTRY: &str = "Denmark";
		let result = Zone::new(NAME.to_string(), COUNTRY);
		assert_eq!(result.name, NAME);
		assert_eq!(result.country, COUNTRY);
	}
}
