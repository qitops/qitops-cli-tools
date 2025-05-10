#[cfg(feature = "ai")]
mod ai_tests {
    use qitops::ai::{AiConfig, AiModelType, AiTestGenerator};
    use qitops::common::TestResult;
    use serde_json::json;

    // Helper function to create a sample test result
    fn create_sample_test_result() -> TestResult {
        TestResult {
            name: "Sample API Test".to_string(),
            status: "passed".to_string(),
            duration: 0.5,
            details: Some(json!({
                "status_code": 200,
                "response_time": 0.45,
                "headers": {
                    "content-type": "application/json"
                },
                "body": {
                    "id": 1,
                    "name": "Test User"
                }
            })),
            timestamp: "2023-05-10T12:00:00Z".to_string(),
        }
    }

    // Test that verifies the AI test generator can be created
    #[tokio::test]
    async fn test_ai_generator_creation() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: Some("/path/to/model.gguf".to_string()),
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: Some("You are a test assistant".to_string()),
        };

        let generator = AiTestGenerator::new(config);
        assert!(generator.config.model_path.is_some());
        assert_eq!(generator.config.temperature, 0.7);
    }

    // Test that verifies prompt creation for API tests
    #[tokio::test]
    async fn test_api_test_prompt_creation() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: None,
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: None,
        };

        let generator = AiTestGenerator::new(config);
        let prompt = generator.create_api_test_prompt("Test the GitHub API");

        assert!(prompt.contains("Test the GitHub API"));
        assert!(prompt.contains("API test"));
    }

    // Test that verifies prompt creation for performance tests
    #[tokio::test]
    async fn test_performance_test_prompt_creation() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: None,
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: None,
        };

        let generator = AiTestGenerator::new(config);
        let prompt = generator.create_performance_test_prompt("Load test for e-commerce site");

        assert!(prompt.contains("Load test for e-commerce site"));
        assert!(prompt.contains("performance test"));
    }

    // Test that verifies analysis prompt creation
    #[tokio::test]
    async fn test_analysis_prompt_creation() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: None,
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: None,
        };

        let generator = AiTestGenerator::new(config);
        let test_result = create_sample_test_result();
        let results_json = serde_json::to_string_pretty(&vec![test_result]).unwrap();
        let prompt = generator.create_analysis_prompt(&results_json);

        assert!(prompt.to_lowercase().contains("analyze"));
        assert!(prompt.contains("Sample API Test"));
    }

    // Test that verifies improvement prompt creation
    #[tokio::test]
    async fn test_improvement_prompt_creation() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: None,
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: None,
        };

        let generator = AiTestGenerator::new(config);
        let test_result = create_sample_test_result();
        let results_json = serde_json::to_string_pretty(&vec![test_result]).unwrap();
        let prompt = generator.create_improvement_prompt(&results_json);

        assert!(prompt.contains("improve"));
        assert!(prompt.contains("Sample API Test"));
    }

    // Test that verifies JSON extraction from LLM output
    #[tokio::test]
    async fn test_json_extraction() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: None,
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: None,
        };

        let generator = AiTestGenerator::new(config);
        let llm_output = r#"
        Some preamble text

        ```json
        {
            "name": "API Test",
            "url": "https://api.example.com",
            "method": "GET"
        }
        ```

        Some concluding text
        "#;

        let json = generator.extract_json_from_output(llm_output);
        assert!(json.contains("API Test"));
        assert!(json.contains("api.example.com"));
        assert!(!json.contains("preamble"));
        assert!(!json.contains("concluding"));
    }
}

// This module contains tests that run regardless of whether the AI feature is enabled
#[cfg(test)]
mod general_ai_tests {
    use qitops::ai::{AiConfig, AiModelType};

    // Test that verifies AiConfig can be serialized and deserialized
    #[test]
    fn test_ai_config_serde() {
        let config = AiConfig {
            model_type: AiModelType::Llama,
            model_path: Some("/path/to/model.gguf".to_string()),
            context_size: 2048,
            temperature: 0.7,
            max_tokens: 1024,
            system_prompt: Some("You are a test assistant".to_string()),
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("Llama")); // Note: enum variants are capitalized in JSON
        assert!(json.contains("/path/to/model.gguf"));

        let deserialized: AiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.context_size, 2048);
        assert_eq!(deserialized.temperature, 0.7);
    }

    // Test that verifies default values are applied correctly
    #[test]
    fn test_ai_config_defaults() {
        let json = r#"{
            "model_type": "Mistral"
        }"#;

        let config: AiConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.context_size, 2048); // Default value
        assert_eq!(config.temperature, 0.7); // Default value
        assert_eq!(config.max_tokens, 1024); // Default value
    }
}
