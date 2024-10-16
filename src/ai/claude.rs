use crate::ai::AI;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

pub struct Claude {
    api_key: String,
    client: Client,
    db_schema: String,
}

impl Claude {
    pub fn new(api_key: String, db_schema: String) -> Self {
        Claude {
            api_key,
            client: Client::new(),
            db_schema,
        }
    }

    async fn make_request(&self, full_prompt: &str, retries: u32) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.anthropic.com/v1/messages";
        
        let payload = json!({
            "model": "claude-3-5-sonnet-20240620",
            "max_tokens": 4096,
            "messages": [
                {
                    "role": "user",
                    "content": full_prompt
                }
            ]
        });

        for attempt in 0..retries {
            let response = self.client
                .post(url)
                .header("X-API-Key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&payload)?)
                .send()
                .await?;

            let response_body: Value = response.json().await?;
            
            if let Some(error) = response_body.get("error") {
                if error["type"] == "overloaded_error" {
                    println!("API overloaded. Retrying in {} seconds...", 2_u64.pow(attempt));
                    sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                    continue;
                }
                return Err(format!("API Error: {:?}", error).into());
            }

            let content = response_body["content"][0]["text"].as_str()
                .ok_or("Failed to extract content from response")?;

            return Ok(content.to_string());
        }

        Err("Max retries reached. Unable to get a response from the API.".into())
    }
}

#[async_trait]
impl AI for Claude {
    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let full_prompt = format!(
            "You are an AI assistant that converts human language queries to SQL based on the following database schema:\n\n{}\n\nRespond only with the SQL query, without any explanation. Human query: {}",
            self.db_schema, prompt
        );

        self.make_request(&full_prompt, 3).await
    }
}
