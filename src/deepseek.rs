use crate::{curl, error::Error, Ask};
use serde_json;

const API_KEY: &str = "DEEPSEEK_API_KEY";
const URL: &str = "https://api.deepseek.com/chat/completions";

pub struct Deepseek;

impl Ask for Deepseek {
    fn name() -> &'static str {
        "Deepseek"
    }

    async fn ask(diff: &str) -> Result<String, Error> {
        let api_key = std::env::var(API_KEY)
            .map_err(|_| Error::new("DEEPSEEK_API_KEY environment variable not set"))?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", api_key.parse().unwrap());

        let body = serde_json::json!({
            "model": "deepseek-chat",
            "messages": [
                {"role": "user", "content": diff},
                {"role": "system", "content": Self::prompt()},
            ],
        });

        let json = curl::post(URL, headers, body.to_string()).await?;

        let answer = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::new("Failed to extract answer from response"))?
            .to_string();
        return Ok(answer);
    }
}
