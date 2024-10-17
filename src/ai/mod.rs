pub mod claude;

use async_trait::async_trait;

#[async_trait]
pub trait AI {
    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    
    // You might want to add more common methods here in the future
    // For example:
    // async fn get_model_name(&self) -> String;
    // async fn get_max_tokens(&self) -> usize;
}

pub use claude::Claude;
