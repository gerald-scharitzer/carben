//! web client for electricity maps API

use std::error::Error;

use reqwest::StatusCode;
use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;

pub const API_ROOT: &str = "https://api.electricitymap.org/";

/// get the response from the API specified by the path
pub fn get(path: &str) -> Result<Response, Box<dyn Error>> {
	let url = format!("{API_ROOT}{path}");
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

	Ok(response)
}

/// get the text content from the API response specified by the path
pub fn text(path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(path)?;
    let text = response.text()?;
    Ok(text)
}
