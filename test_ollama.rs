use qitops::ai::model::{AiModel, OllamaModel};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an Ollama model
    let model = OllamaModel::new("phi")?;
    
    // Generate a test prompt
    let prompt = "Generate a JSON configuration for an API test based on this description: Test the Twitter API to fetch user timeline";
    
    // Run the model
    let response = model.generate(prompt).await?;
    
    // Print the response
    println!("Response from Ollama model:");
    println!("{}", response.content);
    
    Ok(())
}
