use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use crate::error::{Error, Result};
use dotenv::dotenv;
use envsubst::substitute;
use std::{fs};
use std::collections::HashMap;
use serde_json::Value;
use serde::de::DeserializeOwned;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub name: String,
    pub description: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_retries")]
    pub retries: u32,
    pub environment: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestResult {
    pub name: String,
    pub status: String,
    pub duration: f64,
    pub details: Option<serde_json::Value>,
    pub timestamp: String,
}

fn default_timeout() -> u64 {
    30
}

fn default_retries() -> u32 {
    3
}

#[async_trait]
pub trait TestRunner {
    async fn run(&self, config: &(impl serde::Serialize + Send + Sync)) -> Result<TestResult>;
}

pub fn load_config<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path)?;
    let env_vars = std::env::vars().collect::<std::collections::HashMap<_, _>>();
    let substituted = substitute(&content, &env_vars)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Determine test type from file name or path
    let test_type = determine_test_type(path);

    // Parse JSON for validation
    let json_value: Value = serde_json::from_str(&substituted)?;

    // Validate against schema if we have a known test type
    if let Some(test_type) = test_type {
        crate::schema::validate_config(&json_value, test_type)?;
    }

    // Deserialize into the requested type
    Ok(serde_json::from_value(json_value)?)
}

/// Determine the test type from the file path
fn determine_test_type(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name()?.to_str()?;

    if file_name.contains("api_collection") {
        return Some("api_collection");
    } else if file_name.contains("api") {
        return Some("api");
    } else if file_name.contains("performance_enhanced") {
        return Some("performance_enhanced");
    } else if file_name.contains("performance") {
        return Some("performance");
    } else if file_name.contains("security") {
        return Some("security");
    } else if file_name.contains("web") {
        return Some("web");
    } else if file_name.contains("data_driven") {
        // Try to determine from content
        return Some("data_driven");
    }

    None
}

pub fn save_result(result: &TestResult, output_path: &Path) -> Result<()> {
    let content = serde_json::to_string_pretty(result)?;
    std::fs::write(output_path, content)?;
    Ok(())
}
