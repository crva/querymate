pub mod claude;

use async_trait::async_trait;

#[async_trait]
pub trait AI {
    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub use claude::Claude;
