//! Electricity Maps Health API https://static.electricitymaps.com/api/docs/index.html#health

use std::error::Error;

use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

use super::API_ROOT;

#[derive(Deserialize)]
pub struct Monitor {
	pub state: String
}

impl Monitor {
	pub fn new(state: String) -> Self {
		Monitor { state }
	}
}

// API response
#[derive(Deserialize)]
pub struct Health {
	pub monitors: Monitor,
	pub status: String
}

impl Health {
	pub fn new(monitors: Monitor, status: String) -> Self {
		Health { monitors, status }
	}

	pub fn result(&self) -> Result<(), Box<dyn Error>> {
		let state = &self.monitors.state;
		let status = &self.status;
		if state != "ok" || status != "ok" {
			return Err(Box::<dyn Error>::from(format!("status {status} monitor state {state}")));
		}
		Ok(())
	}
}

const PATH: &str = "health";

pub fn get() -> Result<Health, Box<dyn Error>> {
	let url = format!("{API_ROOT}{PATH}");
	let client = Client::new();
	let request = client.get(&url);
	let response = request.send()?;

	let status = response.status();
	if status != StatusCode::OK {
		return Err(Box::<dyn Error>::from(format!("response status {status}")));
	}

	let content_type = &response.headers()[CONTENT_TYPE];
	if content_type != "application/json; charset=utf-8" { // TODO use mime crate
		return Err(Box::<dyn Error>::from(format!("response content type {}", content_type.to_str()?)));
	}

	let health: Health = response.json()?;
	Ok(health)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_health() {
		const STATE: &str = "ok";
		const STATUS: &str = "ok";
		let health = Health::new(Monitor::new(STATE.to_string()), STATUS.to_string());
		assert_eq!(health.monitors.state, STATE);
		assert_eq!(health.status, STATUS);
		assert!(health.result().is_ok());
	}
}
