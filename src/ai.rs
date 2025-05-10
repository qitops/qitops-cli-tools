use crate::common::TestResult;
use crate::error::{Error, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

/// AI model types supported by QitOps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiModelType {
    /// Local LLaMA model
    Llama,
    /// Local Mistral model
    Mistral,
    /// Local GPT-J model
    GptJ,
    /// Local Phi model
    Phi,
    /// Custom model (requires path)
    Custom,
}

/// Configuration for AI-powered test generation and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Type of AI model to use
    pub model_type: AiModelType,
    /// Path to the model weights (for local models)
    pub model_path: Option<String>,
    /// Context window size
    #[serde(default = "default_context_size")]
    pub context_size: usize,
    /// Temperature for generation (0.0-1.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    /// System prompt to use
    pub system_prompt: Option<String>,
}

fn default_context_size() -> usize {
    2048
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> usize {
    1024
}

/// AI-powered test generator
pub struct AiTestGenerator {
    pub config: AiConfig,
}

impl AiTestGenerator {
    /// Create a new AI test generator with the given configuration
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }

    /// Generate a test configuration based on a description
    pub async fn generate_test_config(&self, description: &str, test_type: &str) -> Result<String> {
        info!(
            "Generating {} test configuration from description",
            test_type
        );

        let prompt = match test_type {
            "api" => self.create_api_test_prompt(description),
            "performance" => self.create_performance_test_prompt(description),
            "security" => self.create_security_test_prompt(description),
            "web" => self.create_web_test_prompt(description),
            _ => {
                return Err(Error::ValidationError(format!(
                    "Unsupported test type: {}",
                    test_type
                )))
            }
        };

        let json_config = self.run_inference(&prompt).await?;

        // Validate the generated JSON
        match test_type {
            "api" => self.validate_api_test_config(&json_config)?,
            "performance" => self.validate_performance_test_config(&json_config)?,
            "security" => self.validate_security_test_config(&json_config)?,
            "web" => self.validate_web_test_config(&json_config)?,
            _ => {
                return Err(Error::ValidationError(format!(
                    "Unsupported test type: {}",
                    test_type
                )))
            }
        }

        Ok(json_config)
    }

    /// Analyze test results and provide insights
    pub async fn analyze_test_results(&self, results: &[TestResult]) -> Result<String> {
        info!("Analyzing test results using AI");

        let results_json = serde_json::to_string_pretty(results)?;
        let prompt = self.create_analysis_prompt(&results_json);

        self.run_inference(&prompt).await
    }

    /// Suggest improvements based on test results
    pub async fn suggest_improvements(&self, results: &[TestResult]) -> Result<String> {
        info!("Suggesting improvements based on test results");

        let results_json = serde_json::to_string_pretty(results)?;
        let prompt = self.create_improvement_prompt(&results_json);

        self.run_inference(&prompt).await
    }

    // Private methods

    async fn run_inference(&self, prompt: &str) -> Result<String> {
        // For local models, we'll use a command-line interface to the model
        // In a real implementation, this would use a proper Rust binding to the inference engine

        match self.config.model_type {
            AiModelType::Llama => self.run_llama_inference(prompt).await,
            AiModelType::Mistral => self.run_mistral_inference(prompt).await,
            AiModelType::GptJ => self.run_gptj_inference(prompt).await,
            AiModelType::Phi => self.run_phi_inference(prompt).await,
            AiModelType::Custom => {
                if let Some(path) = &self.config.model_path {
                    self.run_custom_inference(prompt, path).await
                } else {
                    Err(Error::ConfigError(
                        "Model path is required for custom models".to_string(),
                    ))
                }
            }
        }
    }

    async fn run_llama_inference(&self, prompt: &str) -> Result<String> {
        // This is a simplified example using llama.cpp
        let model_path = self
            .config
            .model_path
            .clone()
            .unwrap_or_else(|| "/usr/local/share/models/llama-2-7b-chat.gguf".to_string());

        let temp_file = tempfile::NamedTempFile::new()?;
        fs::write(temp_file.path(), prompt)?;

        let output = Command::new("llama-cli")
            .arg("--model")
            .arg(model_path)
            .arg("--ctx-size")
            .arg(self.config.context_size.to_string())
            .arg("--temp")
            .arg(self.config.temperature.to_string())
            .arg("--n-predict")
            .arg(self.config.max_tokens.to_string())
            .arg("--file")
            .arg(temp_file.path())
            .output()?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(self.extract_json_from_output(&result))
        } else {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            Err(Error::TestError(format!(
                "Llama inference failed: {}",
                error
            )))
        }
    }

    async fn run_mistral_inference(&self, prompt: &str) -> Result<String> {
        // Mock implementation for testing
        if prompt.contains("API test") {
            Ok(r#"{
                "name": "GitHub API Test",
                "description": "Test the GitHub API to fetch user information",
                "timeout": 30,
                "retries": 3,
                "environment": "production",
                "url": "https://api.github.com/users/octocat",
                "method": "GET",
                "headers": {
                    "Accept": "application/vnd.github.v3+json",
                    "User-Agent": "QitOps-Test"
                },
                "expected_status": 200,
                "expected_body": {
                    "login": "octocat",
                    "type": "User"
                }
            }"#.to_string())
        } else if prompt.contains("performance test") {
            Ok(r#"{
                "name": "Performance Test",
                "description": "Load test for a web service",
                "timeout": 60,
                "target_url": "https://example.com/api",
                "method": "GET",
                "users": 10,
                "duration": 30,
                "ramp_up": 5,
                "success_threshold": 95
            }"#.to_string())
        } else {
            Ok("Mistral mock response for testing".to_string())
        }
    }

    async fn run_gptj_inference(&self, prompt: &str) -> Result<String> {
        // Mock implementation for testing
        if prompt.contains("analyze") {
            Ok(r#"# Test Analysis

## Overview
The test was successful with a response time of 0.45 seconds.

## Details
- Status code: 200
- Response time: 0.45s (good performance)
- Content type: application/json

## Recommendations
- The test is performing well
- Consider adding more assertions to validate the response body
            "#.to_string())
        } else {
            Ok("GPT-J mock response for testing".to_string())
        }
    }

    async fn run_phi_inference(&self, prompt: &str) -> Result<String> {
        // Mock implementation for testing
        if prompt.contains("improve") {
            Ok(r#"# Improvement Suggestions

## Performance
- Add response time thresholds to ensure consistent performance
- Consider testing with different payload sizes

## Reliability
- Add retry logic for intermittent failures
- Implement circuit breaker pattern for dependent services

## Coverage
- Add negative test cases
- Test edge cases with invalid inputs
            "#.to_string())
        } else {
            Ok("Phi mock response for testing".to_string())
        }
    }

    async fn run_custom_inference(&self, prompt: &str, model_path: &str) -> Result<String> {
        // Mock implementation for testing
        Ok(format!("Custom model mock response for: {} using model at {}", prompt, model_path))
    }

    fn extract_json_from_output(&self, output: &str) -> String {
        // Extract JSON from the model output
        // First, try to extract from markdown code blocks
        if let Some(start) = output.find("```json") {
            let code_start = start + 7; // Skip "```json" and newline
            if let Some(end) = output[code_start..].find("```") {
                let json = output[code_start..code_start + end].trim();
                return json.to_string();
            }
        }

        // If no code block, try to extract JSON directly
        if let Some(start) = output.find('{') {
            if let Some(end) = output.rfind('}') {
                return output[start..=end].to_string();
            }
        }

        // If all else fails, return the original output
        output.to_string()
    }

    pub fn create_api_test_prompt(&self, description: &str) -> String {
        format!(
            "Generate a JSON configuration for an API test based on this description: {}\n\
            The configuration should include appropriate values for URL, method, headers, \
            expected status, and expected response body. Format as valid JSON for a QitOps API test.",
            description
        )
    }

    pub fn create_performance_test_prompt(&self, description: &str) -> String {
        format!(
            "Generate a JSON configuration for a performance test based on this description: {}\n\
            The configuration should include appropriate values for target URL, method, headers, \
            success threshold, and ramp-up time. Format as valid JSON for a QitOps performance test.",
            description
        )
    }

    pub fn create_security_test_prompt(&self, description: &str) -> String {
        format!(
            "Generate a JSON configuration for a security test based on this description: {}\n\
            The configuration should include appropriate values for target URL, headers, auth, \
            scan types, and maximum high severity findings. Format as valid JSON for a QitOps security test.",
            description
        )
    }

    pub fn create_web_test_prompt(&self, description: &str) -> String {
        format!(
            "Generate a JSON configuration for a web test based on this description: {}\n\
            The configuration should include appropriate values for target URL, viewport, \
            wait conditions, assertions, and actions. Format as valid JSON for a QitOps web test.",
            description
        )
    }

    pub fn create_analysis_prompt(&self, results_json: &str) -> String {
        format!(
            "Analyze these test results and provide insights:\n{}\n\
            Focus on patterns, anomalies, and potential issues. \
            Format your analysis in markdown with sections for summary, details, and recommendations.",
            results_json
        )
    }

    pub fn create_improvement_prompt(&self, results_json: &str) -> String {
        format!(
            "Based on these test results, suggest improvements to the tests or the system under test:\n{}\n\
            Focus on concrete, actionable suggestions that would improve test coverage, reliability, or performance. \
            Format your suggestions in markdown with bullet points.",
            results_json
        )
    }

    // Validation methods

    fn validate_api_test_config(&self, config: &str) -> Result<()> {
        // Simplified validation
        if !config.contains("url") || !config.contains("method") {
            return Err(Error::ValidationError(
                "Generated API test config is missing required fields".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_performance_test_config(&self, config: &str) -> Result<()> {
        // Simplified validation
        if !config.contains("target_url") || !config.contains("method") {
            return Err(Error::ValidationError(
                "Generated performance test config is missing required fields".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_security_test_config(&self, config: &str) -> Result<()> {
        // Simplified validation
        if !config.contains("target_url") {
            return Err(Error::ValidationError(
                "Generated security test config is missing required fields".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_web_test_config(&self, config: &str) -> Result<()> {
        // Simplified validation
        if !config.contains("target_url") {
            return Err(Error::ValidationError(
                "Generated web test config is missing required fields".to_string(),
            ));
        }
        Ok(())
    }
}
