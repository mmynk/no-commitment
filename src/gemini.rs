use crate::{curl, error::Error, Ask};
use serde_json;

const API_KEY: &str = "GOOGLE_API_KEY";
const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

pub struct Gemini;

impl Ask for Gemini {
    fn name() -> &'static str {
        "Gemini"
    }

    async fn ask(diff: &str) -> Result<String, Error> {
        let api_key = std::env::var(API_KEY)
            .map_err(|_| Error::new("GOOGLE_API_KEY environment variable not set"))?;
        let url = format!("{}?key={}", GEMINI_URL, api_key);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let instruction = "You are a terminal based AI assistant.\
            You are supposed to generate commit messages.\
            Your input is the output of `git diff` command.\
            Your output is a commit message. Be concise, there is no need to describe each and every change.\
            For non-code changes (for example README), always generate a single line commit message.\
            Again, be concise.\
            For simple diffs, generate a single line commit message.\
            For complex diffs, generate a multi-line commit message in the format:\
            A brief summary of the changes.\n\
            \n\
            - A detailed description of the changes.\
            \n";

        let body = serde_json::json!({
            "contents": [{
                "role": "model",
                "parts": {
                    "text": instruction,
                },
            }, {
                "role": "user",
                "parts": {
                    "text": diff,
                },
            }],
        });

        let json = curl::post(&url, headers, body.to_string()).await?;

        let answer = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| Error::new("Failed to extract answer from response"))?
            .to_string();
        return Ok(answer);
    }
}
