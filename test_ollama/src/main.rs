use qitops::ai::model::{AiModel, OllamaModel};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Ollama test");

    // Create an Ollama model
    println!("Creating Ollama model with ID: phi");
    let model = OllamaModel::new("phi")?;
    
    // Test API Test Configuration
    println!("Testing API Test Configuration");
    let prompt = "Generate a JSON configuration for an API test based on this description: Test the Twitter API to fetch user timeline";
    let response = model.generate(prompt).await?;
    
    println!("\nAPI Test Configuration Response:");
    println!("{}", response.content);
    
    // Save the response to a file
    std::fs::write("twitter_api_test_real.json", response.content)?;
    println!("Saved to: twitter_api_test_real.json");
    
    Ok(())
}
