pub mod legacy;
pub mod model;
pub mod web;

// Re-export legacy types for backward compatibility
pub use legacy::{AiConfig, AiModelType, AiTestGenerator};

use crate::error::Result;
use log::info;
use std::path::Path;

// Re-export the model module
pub use model::*;

// Function to create an AI model based on the model name and path
pub fn create_model(model_name: &str, model_path: Option<&str>) -> Result<Box<dyn AiModel>> {
    match model_name {
        "custom" => {
            if let Some(path) = model_path {
                if let Some(model_id) = path.strip_prefix("ollama:") {
                    info!("Creating Ollama model with ID: {}", model_id);
                    #[cfg(feature = "ai")]
                    {
                        return Ok(Box::new(model::OllamaModel::new(model_id)?));
                    }
                    #[cfg(not(feature = "ai"))]
                    {
                        Err(crate::error::Error::AiModelError(
                            "AI features are not enabled. Recompile with --features ai".to_string(),
                        ))
                    }
                } else {
                    info!("Creating custom model with path: {}", path);
                    #[cfg(feature = "ai")]
                    {
                        return Ok(Box::new(model::CustomModel::new(path)?));
                    }
                    #[cfg(not(feature = "ai"))]
                    {
                        Err(crate::error::Error::AiModelError(
                            "AI features are not enabled. Recompile with --features ai".to_string(),
                        ))
                    }
                }
            } else {
                Err(crate::error::Error::AiModelError(
                    "Model path is required for custom model".to_string(),
                ))
            }
        }
        "mock" => {
            info!("Creating mock model");
            #[cfg(any(feature = "ai", feature = "ai-mock"))]
            {
                return Ok(Box::new(model::MockModel::new()));
            }
            #[cfg(not(any(feature = "ai", feature = "ai-mock")))]
            {
                Err(crate::error::Error::AiModelError(
                    "AI features are not enabled. Recompile with --features ai or --features ai-mock"
                        .to_string(),
                ))
            }
        }
        _ => {
            Err(crate::error::Error::AiModelError(format!(
                "Unsupported model: {}",
                model_name
            )))
        }
    }
}

// Function to generate a test configuration using AI
pub async fn generate_test_config(
    model: &dyn AiModel,
    test_type: &str,
    description: &str,
    output_path: &Path,
) -> Result<()> {
    info!(
        "Generating {} test configuration for: {}",
        test_type, description
    );

    match test_type {
        "api" => generate_api_test(model, description, output_path).await,
        "performance" => generate_performance_test(model, description, output_path).await,
        "security" => generate_security_test(model, description, output_path).await,
        "web" => {
            #[cfg(feature = "web-testing-ai")]
            {
                let config = web::generate_web_test(model, description, None).await?;
                let json = serde_json::to_string_pretty(&config)?;
                std::fs::write(output_path, json)?;
                Ok(())
            }
            #[cfg(not(feature = "web-testing-ai"))]
            {
                Err(crate::error::Error::AiModelError(
                    "Web testing AI features are not enabled. Recompile with --features web-testing-ai"
                        .to_string(),
                ))
            }
        }
        _ => Err(crate::error::Error::AiModelError(format!(
            "Unsupported test type: {}",
            test_type
        ))),
    }
}

// Function to analyze test results using AI
pub async fn analyze_test_results(
    model: &dyn AiModel,
    results_path: &Path,
    output_path: &Path,
) -> Result<()> {
    info!("Analyzing test results using AI");

    // Read the test results
    let results_str = std::fs::read_to_string(results_path)?;
    let results: serde_json::Value = serde_json::from_str(&results_str)?;

    // Determine the test type from the results
    let test_type = determine_test_type(&results)?;

    // Generate the analysis based on the test type
    let analysis = match test_type {
        "api" => analyze_api_test_results(model, &results).await?,
        "performance" => analyze_performance_test_results(model, &results).await?,
        "security" => analyze_security_test_results(model, &results).await?,
        "web" => {
            #[cfg(feature = "web-testing-ai")]
            {
                // Extract the test configuration from the results
                let config_str = results["config"].to_string();
                let config: crate::web::WebTestConfig = serde_json::from_str(&config_str)?;
                web::analyze_web_test_results(model, &config, &results).await?
            }
            #[cfg(not(feature = "web-testing-ai"))]
            {
                return Err(crate::error::Error::AiModelError(
                    "Web testing AI features are not enabled. Recompile with --features web-testing-ai"
                        .to_string(),
                ));
            }
        }
        _ => {
            return Err(crate::error::Error::AiModelError(format!(
                "Unsupported test type: {}",
                test_type
            )));
        }
    };

    // Write the analysis to the output file
    std::fs::write(output_path, analysis)?;

    Ok(())
}

// Function to generate improvement suggestions for test results using AI
pub async fn generate_test_improvements(
    model: &dyn AiModel,
    results_path: &Path,
    output_path: &Path,
) -> Result<()> {
    info!("Generating test improvement suggestions using AI");

    // Read the test results
    let results_str = std::fs::read_to_string(results_path)?;
    let results: serde_json::Value = serde_json::from_str(&results_str)?;

    // Determine the test type from the results
    let test_type = determine_test_type(&results)?;

    // Generate the improvements based on the test type
    let improvements = match test_type {
        "api" => generate_api_test_improvements(model, &results).await?,
        "performance" => generate_performance_test_improvements(model, &results).await?,
        "security" => generate_security_test_improvements(model, &results).await?,
        "web" => {
            #[cfg(feature = "web-testing-ai")]
            {
                // Extract the test configuration from the results
                let config_str = results["config"].to_string();
                let config: crate::web::WebTestConfig = serde_json::from_str(&config_str)?;
                web::generate_web_test_improvements(model, &config, &results).await?
            }
            #[cfg(not(feature = "web-testing-ai"))]
            {
                return Err(crate::error::Error::AiModelError(
                    "Web testing AI features are not enabled. Recompile with --features web-testing-ai"
                        .to_string(),
                ));
            }
        }
        _ => {
            return Err(crate::error::Error::AiModelError(format!(
                "Unsupported test type: {}",
                test_type
            )));
        }
    };

    // Write the improvements to the output file
    std::fs::write(output_path, improvements)?;

    Ok(())
}

// Helper function to determine the test type from the results
fn determine_test_type(results: &serde_json::Value) -> Result<&'static str> {
    if results["type"] == "api" {
        Ok("api")
    } else if results["type"] == "performance" {
        Ok("performance")
    } else if results["type"] == "security" {
        Ok("security")
    } else if results["type"] == "web" {
        Ok("web")
    } else {
        Err(crate::error::Error::AiModelError(
            "Could not determine test type from results".to_string(),
        ))
    }
}

// Placeholder functions for other test types
async fn generate_api_test(
    _model: &dyn AiModel,
    _description: &str,
    _output_path: &Path,
) -> Result<()> {
    // Implementation would go here
    Ok(())
}

async fn generate_performance_test(
    _model: &dyn AiModel,
    _description: &str,
    _output_path: &Path,
) -> Result<()> {
    // Implementation would go here
    Ok(())
}

async fn generate_security_test(
    _model: &dyn AiModel,
    _description: &str,
    _output_path: &Path,
) -> Result<()> {
    // Implementation would go here
    Ok(())
}

async fn analyze_api_test_results(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("API test analysis placeholder".to_string())
}

async fn analyze_performance_test_results(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("Performance test analysis placeholder".to_string())
}

async fn analyze_security_test_results(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("Security test analysis placeholder".to_string())
}

async fn generate_api_test_improvements(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("API test improvements placeholder".to_string())
}

async fn generate_performance_test_improvements(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("Performance test improvements placeholder".to_string())
}

async fn generate_security_test_improvements(
    _model: &dyn AiModel,
    _results: &serde_json::Value,
) -> Result<String> {
    // Implementation would go here
    Ok("Security test improvements placeholder".to_string())
}
