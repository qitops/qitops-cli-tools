use crate::error::{Error, Result};
use async_trait::async_trait;
use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
}

#[async_trait]
pub trait AiModel: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<ModelResponse>;
}

// Mock model for testing
pub struct MockModel;

impl MockModel {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AiModel for MockModel {
    async fn generate(&self, prompt: &str) -> Result<ModelResponse> {
        info!("Mock model generating response for prompt: {}", prompt);
        
        // Return a simple mock response based on the prompt
        if prompt.contains("web test") {
            Ok(ModelResponse {
                content: r#"{
  "name": "Example Web Test",
  "description": "A simple web test for example.com",
  "target_url": "https://example.com",
  "user_agent": "QitOps-WebTester/1.0",
  "wait_timeout_secs": 30,
  "viewport": {
    "width": 1280,
    "height": 720,
    "device_scale_factor": 1.0,
    "is_mobile": false
  },
  "screenshots": true,
  "assertions": [
    {
      "assertion_type": "title",
      "expected_value": "Example Domain",
      "comparison": "equals"
    },
    {
      "assertion_type": "element",
      "selector": "h1",
      "expected_value": "true"
    },
    {
      "assertion_type": "text",
      "expected_value": "This domain is for use in illustrative examples",
      "comparison": "contains"
    }
  ],
  "actions": []
}"#.to_string(),
            })
        } else if prompt.contains("api test") {
            Ok(ModelResponse {
                content: r#"{
  "name": "Example API Test",
  "description": "A simple API test for JSONPlaceholder",
  "url": "https://jsonplaceholder.typicode.com/posts/1",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "expected_status": 200,
  "expected_body": {
    "userId": 1,
    "id": 1
  }
}"#.to_string(),
            })
        } else if prompt.contains("performance test") {
            Ok(ModelResponse {
                content: r#"{
  "name": "Example Performance Test",
  "description": "A simple performance test for JSONPlaceholder",
  "target_url": "https://jsonplaceholder.typicode.com/posts",
  "method": "GET",
  "headers": {
    "Accept": "application/json"
  },
  "vus": 10,
  "duration_secs": 30,
  "rate_per_second": 5,
  "thresholds": {
    "http_req_duration": ["p(95)<500"]
  }
}"#.to_string(),
            })
        } else if prompt.contains("security test") {
            Ok(ModelResponse {
                content: r#"{
  "name": "Example Security Test",
  "description": "A simple security test for example.com",
  "target_url": "https://example.com",
  "scan_types": ["xss", "sqli", "csrf"],
  "headers": {
    "User-Agent": "QitOps-SecurityTester/1.0"
  },
  "auth": null,
  "depth": 1,
  "max_urls": 10
}"#.to_string(),
            })
        } else if prompt.contains("analyze") {
            Ok(ModelResponse {
                content: "# Test Analysis\n\n## Summary\nThe test was successful with all assertions passing.\n\n## Details\n- Response time: 150ms\n- Status code: 200\n- All assertions passed\n\n## Recommendations\n- Add more assertions to test edge cases\n- Consider adding performance thresholds".to_string(),
            })
        } else if prompt.contains("improve") {
            Ok(ModelResponse {
                content: "# Test Improvement Suggestions\n\n## Additional Assertions\n- Add assertions for response headers\n- Add assertions for specific data fields\n\n## Performance Improvements\n- Add rate limiting to prevent server overload\n- Add caching for repeated requests\n\n## Reliability Improvements\n- Add retry logic for transient failures\n- Add timeout handling".to_string(),
            })
        } else {
            Ok(ModelResponse {
                content: "This is a mock response from the AI model.".to_string(),
            })
        }
    }
}

// Custom model for external LLMs
pub struct CustomModel {
    path: String,
    client: Client,
}

impl CustomModel {
    pub fn new(path: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| Error::AiError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            path: path.to_string(),
            client,
        })
    }
}

#[async_trait]
impl AiModel for CustomModel {
    async fn generate(&self, prompt: &str) -> Result<ModelResponse> {
        info!("Custom model generating response for prompt");
        debug!("Prompt: {}", prompt);
        
        // This would be implemented to call an external API or local model
        // For now, just return a mock response
        Ok(ModelResponse {
            content: format!("Custom model response for: {}", prompt),
        })
    }
}

// Ollama model for local LLMs
pub struct OllamaModel {
    model_id: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaModel {
    pub fn new(model_id: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|e| Error::AiError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            model_id: model_id.to_string(),
            client,
        })
    }
}

#[async_trait]
impl AiModel for OllamaModel {
    async fn generate(&self, prompt: &str) -> Result<ModelResponse> {
        info!("Ollama model generating response for prompt using model: {}", self.model_id);
        debug!("Prompt: {}", prompt);
        
        let request = OllamaRequest {
            model: self.model_id.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };
        
        let response = self.client
            .post("http://localhost:11434/api/generate")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::AiError(format!("Failed to send request to Ollama: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::AiError(format!(
                "Ollama API returned error status: {}",
                response.status()
            )));
        }
        
        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| Error::AiError(format!("Failed to parse Ollama response: {}", e)))?;
        
        Ok(ModelResponse {
            content: ollama_response.response,
        })
    }
}
