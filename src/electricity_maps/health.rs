pub struct Monitor {
	pub state: String
}

impl Monitor {
	pub fn new(state: String) -> Self {
		Monitor { state }
	}
}

pub struct Health {
	pub monitors: Monitor,
	pub status: String
}

impl Health {
	pub fn new(monitors: Monitor, status: String) -> Self {
		Health { monitors, status }
	}
}

pub fn health() -> Health { // TODO call API
	Health::new(Monitor::new("ok".to_string()), "ok".to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_health() {
		const STATE: &str = "ok";
		const STATUS: &str = "ok";
		let result = Health::new(Monitor::new(STATE.to_string()), STATUS.to_string());
		assert_eq!(result.monitors.state, STATE);
		assert_eq!(result.status, STATUS);
	}
}
