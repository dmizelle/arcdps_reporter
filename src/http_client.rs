


use reqwest::blocking::Client;
use serde_json::Value;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the HTTP client
pub fn init_http_client() {
    INIT.call_once(|| {
        // Initialize any global state if needed
    });
}

/// Send combat event JSON to an HTTP endpoint (synchronous version)
pub fn send_combat_event(endpoint: &str, event_json: &Value) -> Result<(), String> {
    init_http_client();

    let client = Client::new();

    match client.post(endpoint)
        .json(event_json)
        .send() {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}


