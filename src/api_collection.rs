use crate::api::ApiTestRunner;
use crate::common::{TestResult, TestRunner};
use crate::error::{Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use log::{info, warn};
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};
use jsonpath_lib as jsonpath;

/// Authentication configuration for API collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionAuth {
    /// Authentication type (basic, bearer, api_key)
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Username for basic auth
    pub username: Option<String>,
    /// Password for basic auth
    pub password: Option<String>,
    /// Token for bearer auth
    pub token: Option<String>,
    /// Key name for API key auth
    pub key_name: Option<String>,
    /// Key value for API key auth
    pub key_value: Option<String>,
    /// Header or query parameter for API key
    pub key_in: Option<String>,
}

/// Default request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionDefaults {
    /// Default headers for all requests
    pub headers: Option<HashMap<String, String>>,
    /// Default timeout in seconds
    pub timeout: Option<u64>,
    /// Default number of retries
    pub retries: Option<u32>,
}

/// Run options for the collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRunOptions {
    /// Run requests sequentially (true) or in parallel (false)
    pub sequential: Option<bool>,
    /// Stop on first failure
    pub stop_on_failure: Option<bool>,
    /// Delay between requests in milliseconds
    pub delay_between_requests_ms: Option<u64>,
}

/// A single request in a collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRequest {
    /// Request name
    pub name: String,
    /// Request description
    pub description: Option<String>,
    /// Request ID (used for dependencies)
    pub id: Option<String>,
    /// Request URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Request headers
    pub headers: Option<HashMap<String, String>>,
    /// Request body
    pub body: Option<Value>,
    /// Expected HTTP status code
    pub expected_status: Option<u16>,
    /// Expected response body
    pub expected_body: Option<Value>,
    /// Expected response body type (object, array, etc.)
    pub expected_body_type: Option<String>,
    /// Request dependencies (IDs of requests that must be executed before this one)
    pub depends_on: Option<Vec<String>>,
    /// Variables to capture from the response
    pub capture: Option<HashMap<String, String>>,
}

/// API collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCollection {
    /// Collection name
    pub name: String,
    /// Collection description
    pub description: Option<String>,
    /// Collection version
    pub version: Option<String>,
    /// Collection variables
    pub variables: Option<HashMap<String, String>>,
    /// Collection authentication
    pub auth: Option<CollectionAuth>,
    /// Default request configuration
    pub defaults: Option<CollectionDefaults>,
    /// Collection requests
    pub requests: Vec<CollectionRequest>,
    /// Environment-specific variables
    pub environments: Option<HashMap<String, HashMap<String, String>>>,
    /// Run options
    pub run_options: Option<CollectionRunOptions>,
}

/// Result of a collection run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResult {
    /// Collection name
    pub name: String,
    /// Collection status (passed, failed, error)
    pub status: String,
    /// Total duration in seconds
    pub duration: f64,
    /// Results of individual requests
    pub request_results: Vec<TestResult>,
    /// Timestamp
    pub timestamp: String,
    /// Captured variables
    pub variables: HashMap<String, String>,
}

/// API collection runner
pub struct ApiCollectionRunner {
    client: Client,
    api_runner: ApiTestRunner,
}

impl ApiCollectionRunner {
    /// Create a new API collection runner
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            api_runner: ApiTestRunner::new(),
        }
    }

    /// Load an API collection from a file
    pub fn load_collection(path: &Path) -> Result<ApiCollection> {
        let content = std::fs::read_to_string(path)?;
        let collection: ApiCollection = serde_json::from_str(&content)?;
        Ok(collection)
    }

    /// Run an API collection
    pub async fn run_collection(&self, collection: &ApiCollection, environment: &str) -> Result<CollectionResult> {
        let start = Instant::now();

        // Initialize variables with collection variables and environment variables
        let mut variables = HashMap::new();
        if let Some(collection_vars) = &collection.variables {
            variables.extend(collection_vars.clone());
        }

        // Add environment-specific variables
        if let Some(environments) = &collection.environments {
            if let Some(env_vars) = environments.get(environment) {
                variables.extend(env_vars.clone());
            }
        }

        // Add environment variables from the system
        for (key, value) in std::env::vars() {
            variables.insert(key, value);
        }

        info!("Running collection: {} with {} requests", collection.name, collection.requests.len());
        info!("Using environment: {}", environment);

        // Determine run options
        let sequential = collection.run_options.as_ref()
            .and_then(|opts| opts.sequential)
            .unwrap_or(true);

        let stop_on_failure = collection.run_options.as_ref()
            .and_then(|opts| opts.stop_on_failure)
            .unwrap_or(true);

        let delay = collection.run_options.as_ref()
            .and_then(|opts| opts.delay_between_requests_ms)
            .unwrap_or(0);

        // Track completed requests and their results
        let mut completed_requests = HashMap::new();
        let mut request_results = Vec::new();

        // Run requests
        if sequential {
            // Sequential execution
            for request in &collection.requests {
                // Check dependencies
                if let Some(dependencies) = &request.depends_on {
                    for dep_id in dependencies {
                        if !completed_requests.contains_key(dep_id) {
                            return Err(Error::ValidationError(format!(
                                "Request '{}' depends on '{}', which has not been executed",
                                request.name, dep_id
                            )));
                        }
                    }
                }

                // Execute request
                let result = self.execute_request(request, &collection, &variables).await?;

                // Store result
                if let Some(id) = &request.id {
                    completed_requests.insert(id.clone(), result.clone());
                }
                request_results.push(result.clone());

                // Update variables with captured values
                if let Some(captures) = &request.capture {
                    if let Some(details) = &result.details {
                        if let Some(response_body) = details.get("response_body") {
                            for (var_name, json_path) in captures {
                                if let Ok(values) = jsonpath::select(response_body, json_path) {
                                    if !values.is_empty() {
                                        let value = &values[0];
                                        let value_str = match value {
                                            Value::String(s) => s.clone(),
                                            Value::Number(n) => n.to_string(),
                                            Value::Bool(b) => b.to_string(),
                                            Value::Null => "null".to_string(),
                                            Value::Object(_) | Value::Array(_) => serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string()),
                                        };

                                        info!("Captured variable '{}' with value: {}", var_name, value_str);
                                        variables.insert(var_name.clone(), value_str);
                                    } else {
                                        warn!("JSONPath '{}' matched no values in response", json_path);
                                    }
                                } else {
                                    warn!("Failed to evaluate JSONPath '{}' on response", json_path);
                                }
                            }
                        } else {
                            warn!("No response body found in result details");
                        }
                    } else {
                        warn!("No details found in result");
                    }
                }

                // Check if we should stop on failure
                if stop_on_failure && result.status != "passed" {
                    break;
                }

                // Delay between requests if specified
                if delay > 0 {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        } else {
            // Parallel execution (not implemented yet)
            return Err(Error::ValidationError("Parallel execution not yet implemented".to_string()));
        }

        // Calculate overall status
        let status = if request_results.iter().all(|r| r.status == "passed") {
            "passed"
        } else {
            "failed"
        };

        let duration = start.elapsed().as_secs_f64();

        Ok(CollectionResult {
            name: collection.name.clone(),
            status: status.to_string(),
            duration,
            request_results,
            timestamp: Utc::now().to_rfc3339(),
            variables,
        })
    }

    /// Execute a single request from the collection
    async fn execute_request(&self, request: &CollectionRequest, collection: &ApiCollection, variables: &HashMap<String, String>) -> Result<TestResult> {
        let start = Instant::now();

        // Interpolate variables in URL
        let url = self.interpolate_variables(&request.url, variables)?;

        // Determine HTTP method
        let method = Method::from_bytes(request.method.as_bytes())
            .map_err(|e| Error::ValidationError(format!("Invalid HTTP method: {}", e)))?;

        // Build request with default timeout from collection or default value
        let timeout = collection.defaults.as_ref()
            .and_then(|d| d.timeout)
            .unwrap_or(30);

        let mut req_builder = self.client.request(method, &url)
            .timeout(Duration::from_secs(timeout));

        // Add headers from collection defaults
        if let Some(defaults) = &collection.defaults {
            if let Some(default_headers) = &defaults.headers {
                for (key, value) in default_headers {
                    let interpolated_value = self.interpolate_variables(value, variables)?;
                    req_builder = req_builder.header(key, interpolated_value);
                }
            }
        }

        // Add headers from request (overriding defaults)
        if let Some(headers) = &request.headers {
            for (key, value) in headers {
                let interpolated_value = self.interpolate_variables(value, variables)?;
                req_builder = req_builder.header(key, interpolated_value);
            }
        }

        // Add authentication if specified at collection level
        if let Some(auth) = &collection.auth {
            req_builder = self.add_authentication(req_builder, auth, variables)?;
        }

        // Add request body if specified
        if let Some(body) = &request.body {
            // Interpolate variables in the body
            let body_str = serde_json::to_string(body)?;
            let interpolated_body_str = self.interpolate_variables(&body_str, variables)?;
            let interpolated_body: Value = serde_json::from_str(&interpolated_body_str)?;
            req_builder = req_builder.json(&interpolated_body);
        }

        // Send the request
        info!("Sending {} request to {}", request.method, url);
        let response = req_builder.send().await?;

        // Process response
        let status = response.status();
        let headers = response.headers().clone();
        let response_body_text = response.text().await?;
        let response_body: Value = serde_json::from_str(&response_body_text).unwrap_or_else(|_| {
            // If not valid JSON, return as string
            Value::String(response_body_text.clone())
        });

        // Validate response
        let duration = start.elapsed().as_secs_f64();
        let validation_result = self.validate_response(
            &status,
            &headers,
            &response_body,
            request,
            duration
        )?;

        // Create response details
        let mut details = json!({
            "status_code": status.as_u16(),
            "response_time": duration,
            "response_body": response_body,
            "headers": headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<std::collections::HashMap<_, _>>()
        });

        // Add validation results if any
        let has_validation_issues = if let Some(ref issues) = validation_result {
            details["validation_issues"] = json!(issues);
            true
        } else {
            false
        };

        // Determine test status
        let test_status = if status.is_success() && !has_validation_issues {
            "passed"
        } else {
            "failed"
        };

        Ok(TestResult {
            name: request.name.clone(),
            status: test_status.to_string(),
            duration,
            details: Some(details),
            timestamp: Utc::now().to_rfc3339(),
        })
    }

    /// Add authentication to the request
    fn add_authentication(&self, mut req_builder: reqwest::RequestBuilder, auth: &CollectionAuth, variables: &HashMap<String, String>) -> Result<reqwest::RequestBuilder> {
        match auth.auth_type.as_str() {
            "basic" => {
                let username = auth.username.as_ref()
                    .ok_or_else(|| Error::ValidationError("Username required for basic auth".to_string()))?;
                let password = auth.password.as_ref()
                    .ok_or_else(|| Error::ValidationError("Password required for basic auth".to_string()))?;

                let interpolated_username = self.interpolate_variables(username, variables)?;
                let interpolated_password = self.interpolate_variables(password, variables)?;

                req_builder = req_builder.basic_auth(interpolated_username, Some(interpolated_password));
            },
            "bearer" => {
                let token = auth.token.as_ref()
                    .ok_or_else(|| Error::ValidationError("Token required for bearer auth".to_string()))?;

                let interpolated_token = self.interpolate_variables(token, variables)?;
                req_builder = req_builder.bearer_auth(interpolated_token);
            },
            "api_key" => {
                let key_name = auth.key_name.as_ref()
                    .ok_or_else(|| Error::ValidationError("Key name required for API key auth".to_string()))?;
                let key_value = auth.key_value.as_ref()
                    .ok_or_else(|| Error::ValidationError("Key value required for API key auth".to_string()))?;

                let interpolated_key_value = self.interpolate_variables(key_value, variables)?;

                // Determine if the key should be in header or query parameter
                match auth.key_in.as_deref() {
                    Some("query") => {
                        // Add as query parameter
                        req_builder = req_builder.query(&[(key_name, interpolated_key_value)]);
                    },
                    _ => {
                        // Default to header
                        req_builder = req_builder.header(key_name, interpolated_key_value);
                    }
                }
            },
            _ => {
                return Err(Error::ValidationError(format!("Unsupported authentication type: {}", auth.auth_type)));
            }
        }

        Ok(req_builder)
    }

    /// Interpolate variables in a string
    fn interpolate_variables(&self, input: &str, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = input.to_string();

        // Find all variable references in the format {{variable_name}}
        let re = regex::Regex::new(r"\{\{([^{}]+)\}\}").unwrap();

        // Replace each variable reference with its value
        for capture in re.captures_iter(input) {
            let var_name = &capture[1];
            let var_placeholder = &capture[0]; // The full {{variable_name}}

            if let Some(value) = variables.get(var_name) {
                result = result.replace(var_placeholder, value);
            } else {
                // Variable not found - could either error or leave as is
                warn!("Variable '{}' not found during interpolation", var_name);
            }
        }

        Ok(result)
    }

    /// Validate response against expected values
    fn validate_response(
        &self,
        status: &reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
        body: &Value,
        request: &CollectionRequest,
        duration: f64
    ) -> Result<Option<Vec<String>>> {
        let mut validation_issues = Vec::new();

        // Validate status code if expected
        if let Some(expected_status) = request.expected_status {
            if status.as_u16() != expected_status {
                validation_issues.push(format!(
                    "Status code mismatch. Expected: {}, Got: {}",
                    expected_status,
                    status.as_u16()
                ));
            }
        }

        // Validate body type if expected
        if let Some(expected_type) = &request.expected_body_type {
            let actual_type = match body {
                Value::Object(_) => "object",
                Value::Array(_) => "array",
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::Bool(_) => "boolean",
                Value::Null => "null",
            };

            if actual_type != expected_type {
                validation_issues.push(format!(
                    "Body type mismatch. Expected: {}, Got: {}",
                    expected_type,
                    actual_type
                ));
            }
        }

        // Validate specific body fields if expected
        if let Some(expected_body) = &request.expected_body {
            if let Some(expected_obj) = expected_body.as_object() {
                for (key, expected_value) in expected_obj {
                    if let Some(actual_value) = body.get(key) {
                        if actual_value != expected_value {
                            validation_issues.push(format!(
                                "Field '{}' mismatch. Expected: {:?}, Got: {:?}",
                                key, expected_value, actual_value
                            ));
                        }
                    } else {
                        validation_issues.push(format!(
                            "Expected field '{}' not found in response body",
                            key
                        ));
                    }
                }
            }
        }

        if validation_issues.is_empty() {
            Ok(None)
        } else {
            Ok(Some(validation_issues))
        }
    }
}
