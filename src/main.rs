use env_logger::Env;
use log::info;
use qitops::ai::model::{AiModel, OllamaModel};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    info!("Starting Ollama test");

    // Create an Ollama model
    info!("Creating Ollama model with ID: phi");
    let model = OllamaModel::new("phi")?;

    // Test 1: Generate API Test Configuration
    info!("Test 1: Generate API Test Configuration");
    let prompt = "Generate a JSON configuration for an API test based on this description: Test the Twitter API to fetch user timeline";
    let response = model.generate(prompt).await?;

    println!("\nAPI Test Configuration Response:");
    println!("{}", response.content);

    // Save the response to a file
    std::fs::write("twitter_api_test_real.json", response.content)?;
    println!("Saved to: twitter_api_test_real.json");

    // Test 2: Generate Performance Test Configuration
    info!("Test 2: Generate Performance Test Configuration");
    let prompt = "Generate a JSON configuration for a performance test based on this description: Load test for an e-commerce checkout API with 100 concurrent users";
    let response = model.generate(prompt).await?;

    println!("\nPerformance Test Configuration Response:");
    println!("{}", response.content);

    // Save the response to a file
    std::fs::write("ecommerce_perf_test_real.json", response.content)?;
    println!("Saved to: ecommerce_perf_test_real.json");

    // Create sample test results for analysis
    info!("Creating sample test results for analysis");
    let sample_results = r#"[
      {
        "name": "GitHub User API Test",
        "status": "success",
        "duration": 0.19,
        "details": {
          "test_id": "api-test-1",
          "name": "GitHub User API Test",
          "description": "Test the GitHub API to fetch user information",
          "timestamp": "2025-05-10T21:15:00Z",
          "duration_ms": 190,
          "status": "success",
          "url": "https://api.github.com/users/octocat",
          "method": "GET",
          "request_headers": {
            "Accept": "application/vnd.github.v3+json",
            "User-Agent": "QitOps-Test"
          },
          "response_status": 200,
          "response_headers": {
            "content-type": "application/json; charset=utf-8",
            "cache-control": "public, max-age=60, s-maxage=60"
          },
          "assertions": [
            {
              "type": "status",
              "expected": 200,
              "actual": 200,
              "result": "pass"
            },
            {
              "type": "json",
              "path": "$.login",
              "expected": "octocat",
              "actual": "octocat",
              "result": "pass"
            }
          ]
        },
        "timestamp": "2025-05-10T21:15:00Z"
      },
      {
        "name": "GitHub Non-existent User Test",
        "status": "failure",
        "duration": 0.18,
        "details": {
          "test_id": "api-test-2",
          "name": "GitHub Non-existent User Test",
          "description": "Test the GitHub API with a non-existent user",
          "timestamp": "2025-05-10T21:15:02Z",
          "duration_ms": 180,
          "status": "failure",
          "url": "https://api.github.com/users/non-existent-user-12345",
          "method": "GET",
          "request_headers": {
            "Accept": "application/vnd.github.v3+json",
            "User-Agent": "QitOps-Test"
          },
          "response_status": 404,
          "response_headers": {
            "content-type": "application/json; charset=utf-8",
            "cache-control": "public, max-age=60, s-maxage=60"
          },
          "assertions": [
            {
              "type": "status",
              "expected": 200,
              "actual": 404,
              "result": "fail"
            }
          ],
          "error": "Expected status 200 but got 404"
        },
        "timestamp": "2025-05-10T21:15:02Z"
      }
    ]"#;

    std::fs::write("sample_test_results_real.json", sample_results)?;

    // Test 3: Analyze Test Results
    info!("Test 3: Analyze Test Results");
    let prompt = format!(
        "Analyze these test results and provide insights:\n\n{}",
        sample_results
    );
    let response = model.generate(&prompt).await?;

    println!("\nTest Analysis Response:");
    println!("{}", response.content);

    // Save the response to a file
    std::fs::write("test_analysis_real.md", response.content)?;
    println!("Saved to: test_analysis_real.md");

    // Test 4: Generate Improvement Suggestions
    info!("Test 4: Generate Improvement Suggestions");
    let prompt = format!(
        "Based on these test results, suggest improvements to the tests or the system under test:\n\n{}",
        sample_results
    );
    let response = model.generate(&prompt).await?;

    println!("\nImprovement Suggestions Response:");
    println!("{}", response.content);

    // Save the response to a file
    std::fs::write("test_improvements_real.md", response.content)?;
    println!("Saved to: test_improvements_real.md");

    println!("\nAll tests completed successfully!");
    println!("Generated files:");
    println!("- twitter_api_test_real.json");
    println!("- ecommerce_perf_test_real.json");
    println!("- sample_test_results_real.json");
    println!("- test_analysis_real.md");
    println!("- test_improvements_real.md");

    Ok(())
}
