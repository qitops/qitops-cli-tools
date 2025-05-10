use crate::common::{TestConfig, TestResult, TestRunner};
use crate::error::{Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use jsonschema::JSONSchema;
use log::{info, warn};
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RetryConfig {
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_initial_delay")]
    pub initial_delay_ms: u64,
    #[serde(default = "default_max_delay")]
    pub max_delay_ms: u64,
    #[serde(default = "default_retry_status_codes")]
    pub retry_status_codes: Vec<u16>,
    #[serde(default)]
    pub retry_on_timeout: bool,
    #[serde(default)]
    pub retry_on_connection_error: bool,
}

fn default_max_retries() -> u32 {
    3
}

fn default_initial_delay() -> u64 {
    100
}

fn default_max_delay() -> u64 {
    5000
}

fn default_retry_status_codes() -> Vec<u16> {
    vec![408, 429, 500, 502, 503, 504]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTestConfig {
    #[serde(flatten)]
    pub base: TestConfig,
    pub url: String,
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
    pub expected_status: Option<u16>,
    pub expected_body: Option<serde_json::Value>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default)]
    pub max_response_time: Option<u64>,
    pub expected_headers: Option<serde_json::Value>,
    pub json_schema: Option<serde_json::Value>,
    #[serde(default)]
    pub retry: RetryConfig,
}

fn default_timeout() -> u64 {
    30 // Default timeout of 30 seconds
}

pub struct ApiTestRunner {
    client: Client,
}

impl ApiTestRunner {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }
}

impl Default for ApiTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiTestRunner {
    async fn execute_request_with_retry(&self, config: &ApiTestConfig) -> Result<Response> {
        let retry_strategy = ExponentialBackoff::from_millis(config.retry.initial_delay_ms)
            .max_delay(Duration::from_millis(config.retry.max_delay_ms))
            .map(jitter) // Add jitter to prevent thundering herd
            .take(config.retry.max_retries as usize);

        let mut attempt = 1;
        let result = Retry::spawn(retry_strategy, || {
            let attempt_num = attempt;
            attempt += 1;
            async move {
                info!(
                    "Attempt {} of {}",
                    attempt_num,
                    config.retry.max_retries + 1
                );
                match self.execute_request(config).await {
                    Ok(response) => {
                        let status = response.status();
                        let status_code = status.as_u16();

                        // Check if we should retry based on status code
                        if config.retry.retry_status_codes.contains(&status_code) {
                            warn!(
                                "Request failed with status {} on attempt {}, retrying...",
                                status_code, attempt_num
                            );
                            Err(Error::TestError(format!(
                                "Retryable status code: {} (attempt {}/{})",
                                status_code,
                                attempt_num,
                                config.retry.max_retries + 1
                            )))
                        } else {
                            Ok(response)
                        }
                    }
                    Err(e) => {
                        // Check if we should retry based on error type
                        let should_retry = match &e {
                            Error::RequestError(req_err) => {
                                if req_err.is_timeout() && config.retry.retry_on_timeout {
                                    warn!(
                                        "Request timed out on attempt {}, retrying...",
                                        attempt_num
                                    );
                                    true
                                } else if req_err.is_connect()
                                    && config.retry.retry_on_connection_error
                                {
                                    warn!(
                                        "Connection error on attempt {}, retrying...",
                                        attempt_num
                                    );
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        };

                        if should_retry {
                            Err(e)
                        } else {
                            Err(Error::TestError(format!(
                                "Non-retryable error on attempt {}: {}",
                                attempt_num, e
                            )))
                        }
                    }
                }
            }
        })
        .await;

        match result {
            Ok(response) => {
                info!("Request succeeded after {} attempts", attempt - 1);
                Ok(response)
            }
            Err(e) => {
                warn!(
                    "All {} retry attempts failed: {}",
                    config.retry.max_retries + 1,
                    e
                );
                Err(e)
            }
        }
    }

    async fn execute_request(&self, config: &ApiTestConfig) -> Result<Response> {
        let method = Method::from_bytes(config.method.as_bytes())
            .map_err(|e| Error::ValidationError(format!("Invalid HTTP method: {}", e)))?;

        let mut request = self
            .client
            .request(method, &config.url)
            .timeout(Duration::from_secs(config.timeout));

        if let Some(headers) = &config.headers {
            if let Some(headers_obj) = headers.as_object() {
                for (key, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        request = request.header(key, value_str);
                    }
                }
            }
        }

        if let Some(body) = &config.body {
            request = request.json(body);
        }

        info!("Sending {} request to {}", config.method, config.url);
        let response = request.send().await?;
        Ok(response)
    }

    async fn validate_response(
        &self,
        response: Response,
        config: &ApiTestConfig,
        duration: f64,
    ) -> Result<serde_json::Value> {
        // Validate response time if specified
        if let Some(max_time) = config.max_response_time {
            if duration > max_time as f64 {
                return Err(Error::TestError(format!(
                    "Response time exceeded maximum allowed time. Expected: {}s, Got: {:.2}s",
                    max_time, duration
                )));
            }
        }

        // Validate status code if specified
        if let Some(expected_status) = config.expected_status {
            if response.status().as_u16() != expected_status {
                return Err(Error::TestError(format!(
                    "Expected status {} but got {}",
                    expected_status,
                    response.status().as_u16()
                )));
            }
        }

        // Validate response headers if specified
        if let Some(expected_headers) = &config.expected_headers {
            if let Some(expected_obj) = expected_headers.as_object() {
                for (key, expected_value) in expected_obj {
                    if let Some(actual_value) = response.headers().get(key) {
                        let actual_str = actual_value.to_str().unwrap_or("");
                        let expected_str = expected_value.as_str().unwrap_or("");
                        if actual_str != expected_str {
                            return Err(Error::TestError(format!(
                                "Response header mismatch for '{}'. Expected: {}, Got: {}",
                                key, expected_str, actual_str
                            )));
                        }
                    } else {
                        return Err(Error::TestError(format!(
                            "Expected header '{}' not found in response",
                            key
                        )));
                    }
                }
            }
        }

        let body_text = response.text().await?;
        let actual_body: serde_json::Value = match serde_json::from_str(&body_text) {
            Ok(json) => json,
            Err(_) => {
                return Err(Error::TestError(format!(
                    "Response is not valid JSON: {}",
                    body_text
                )));
            }
        };

        // Validate against JSON Schema if specified
        if let Some(schema) = &config.json_schema {
            let compiled_schema = JSONSchema::compile(schema)
                .map_err(|e| Error::ValidationError(format!("Invalid JSON Schema: {}", e)))?;

            let validation_result = compiled_schema.validate(&actual_body);
            if let Err(errors) = validation_result {
                let error_messages: Vec<String> = errors.map(|e| format!("{}", e)).collect();
                return Err(Error::TestError(format!(
                    "JSON Schema validation failed:\n{}",
                    error_messages.join("\n")
                )));
            }
        }

        // Validate specific fields if expected_body is specified
        if let Some(expected_body) = &config.expected_body {
            if let Some(expected_obj) = expected_body.as_object() {
                for (key, expected_value) in expected_obj {
                    if let Some(actual_value) = actual_body.get(key) {
                        if actual_value != expected_value {
                            return Err(Error::TestError(format!(
                                "Field '{}' mismatch. Expected: {:?}, Got: {:?}",
                                key, expected_value, actual_value
                            )));
                        }
                    } else {
                        return Err(Error::TestError(format!(
                            "Expected field '{}' not found in response body",
                            key
                        )));
                    }
                }
            }
        }

        Ok(actual_body)
    }
}

#[async_trait]
impl TestRunner for ApiTestRunner {
    async fn run(&self, config: &(impl serde::Serialize + Send + Sync)) -> Result<TestResult> {
        let config = serde_json::from_value::<ApiTestConfig>(serde_json::to_value(config)?)?;
        let start = Instant::now();

        match self.execute_request_with_retry(&config).await {
            Ok(response) => {
                let duration = start.elapsed().as_secs_f64();
                let status = response.status();
                let headers = response.headers().clone();

                match self.validate_response(response, &config, duration).await {
                    Ok(body) => Ok(TestResult {
                        name: config.base.name,
                        status: "passed".to_string(),
                        duration,
                        details: Some(serde_json::json!({
                            "status_code": status.as_u16(),
                            "response_time": duration,
                            "headers": headers
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                                .collect::<std::collections::HashMap<_, _>>(),
                            "body": body
                        })),
                        timestamp: Utc::now().to_rfc3339(),
                    }),
                    Err(e) => Ok(TestResult {
                        name: config.base.name,
                        status: "failed".to_string(),
                        duration,
                        details: Some(serde_json::json!({
                            "error": e.to_string(),
                            "status_code": status.as_u16(),
                            "response_time": duration
                        })),
                        timestamp: Utc::now().to_rfc3339(),
                    }),
                }
            }
            Err(e) => {
                let duration = start.elapsed().as_secs_f64();
                Ok(TestResult {
                    name: config.base.name,
                    status: "failed".to_string(),
                    duration,
                    details: Some(serde_json::json!({
                        "error": e.to_string(),
                        "response_time": duration
                    })),
                    timestamp: Utc::now().to_rfc3339(),
                })
            }
        }
    }
}
