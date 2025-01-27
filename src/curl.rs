use reqwest::{header::HeaderMap, Client};
use serde_json;

use crate::error::Error;

pub async fn post(url: &str, headers: HeaderMap, body: String) -> Result<serde_json::Value, Error> {
    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| Error::new(&format!("Failed to send request: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        return Err(Error::new(&format!(
            "Request failed with status code: {}",
            status
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| Error::new(&format!("Failed to read response: {}", e)))?;
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| Error::new(&format!("Failed to parse response: {}", e)))?;
    return Ok(json);
}
