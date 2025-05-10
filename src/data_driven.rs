use crate::error::{Error, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// Data source types for data-driven testing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceType {
    /// CSV file data source
    Csv,
    /// JSON file data source
    Json,
    /// Inline data source (defined in the test configuration)
    Inline,
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Type of data source
    #[serde(rename = "type")]
    pub source_type: DataSourceType,

    /// Path to the data file (for CSV and JSON types)
    pub file_path: Option<String>,

    /// Delimiter for CSV files (default: ",")
    #[serde(default = "default_delimiter")]
    pub delimiter: String,

    /// Whether the CSV file has a header row (default: true)
    #[serde(default = "default_true")]
    pub has_header: bool,

    /// JSON path for extracting data from JSON files (default: "$")
    #[serde(default = "default_json_path")]
    pub json_path: String,

    /// Inline data (for inline type)
    pub data: Option<Vec<HashMap<String, Value>>>,
}

fn default_delimiter() -> String {
    ",".to_string()
}

fn default_true() -> bool {
    true
}

fn default_json_path() -> String {
    "$".to_string()
}

/// Data-driven test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDrivenConfig {
    /// Data source configuration
    pub data_source: DataSource,

    /// Whether to stop on first failure (default: false)
    #[serde(default)]
    pub stop_on_failure: bool,

    /// Maximum number of iterations to run (default: all)
    pub max_iterations: Option<usize>,

    /// Template placeholders to replace in the test configuration
    pub placeholders: Option<Vec<String>>,
}

/// Data-driven test runner
pub struct DataDrivenRunner {
    config: DataDrivenConfig,
    data_rows: Vec<HashMap<String, String>>,
}

/// Replace placeholders in a JSON value with data from a HashMap
pub fn replace_placeholders(value: &mut Value, data: &HashMap<String, String>) {
    match value {
        Value::String(s) => {
            let mut result = s.clone();
            for (key, val) in data {
                let placeholder = format!("{{{{{}}}}}", key);
                result = result.replace(&placeholder, val);
            }
            *s = result;
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                replace_placeholders(v, data);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                replace_placeholders(v, data);
            }
        }
        _ => {}
    }
}

impl DataDrivenRunner {
    /// Create a new data-driven test runner
    pub fn new(config: DataDrivenConfig) -> Result<Self> {
        let data_rows = Self::load_data(&config.data_source)?;

        Ok(Self { config, data_rows })
    }

    /// Load data from the specified data source
    fn load_data(data_source: &DataSource) -> Result<Vec<HashMap<String, String>>> {
        match data_source.source_type {
            DataSourceType::Csv => Self::load_csv_data(data_source),
            DataSourceType::Json => Self::load_json_data(data_source),
            DataSourceType::Inline => Self::load_inline_data(data_source),
        }
    }

    /// Load data from a CSV file
    fn load_csv_data(data_source: &DataSource) -> Result<Vec<HashMap<String, String>>> {
        let file_path = data_source.file_path.as_ref().ok_or_else(|| {
            Error::ValidationError("File path is required for CSV data source".to_string())
        })?;

        let file = File::open(file_path).map_err(|e| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open CSV file: {}", e),
            ))
        })?;

        let reader = BufReader::new(file);
        let delimiter = data_source.delimiter.chars().next().unwrap_or(',');

        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(delimiter as u8)
            .has_headers(data_source.has_header)
            .from_reader(reader);

        let headers = if data_source.has_header {
            csv_reader
                .headers()
                .map_err(|e| Error::ValidationError(format!("Failed to read CSV headers: {}", e)))?
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<String>>()
        } else {
            // If no headers, use column indices as headers
            let first_record = csv_reader
                .records()
                .next()
                .ok_or_else(|| Error::ValidationError("CSV file is empty".to_string()))?
                .map_err(|e| Error::ValidationError(format!("Failed to read CSV record: {}", e)))?;

            (0..first_record.len())
                .map(|i| format!("column{}", i))
                .collect::<Vec<String>>()
        };

        // Reopen the file to start reading from the beginning
        let file = File::open(file_path).map_err(|e| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to reopen CSV file: {}", e),
            ))
        })?;

        // Create a new CSV reader directly from the file
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(delimiter as u8)
            .has_headers(data_source.has_header)
            .from_reader(file);

        let mut data_rows = Vec::new();

        for result in csv_reader.records() {
            let record = result
                .map_err(|e| Error::ValidationError(format!("Failed to read CSV record: {}", e)))?;

            let mut row = HashMap::new();
            for (i, value) in record.iter().enumerate() {
                if i < headers.len() {
                    row.insert(headers[i].clone(), value.to_string());
                }
            }

            data_rows.push(row);
        }

        Ok(data_rows)
    }

    /// Load data from a JSON file
    fn load_json_data(data_source: &DataSource) -> Result<Vec<HashMap<String, String>>> {
        let file_path = data_source.file_path.as_ref().ok_or_else(|| {
            Error::ValidationError("File path is required for JSON data source".to_string())
        })?;

        let file = File::open(file_path).map_err(|e| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open JSON file: {}", e),
            ))
        })?;

        let json_value: Value = serde_json::from_reader(file)
            .map_err(|e| Error::ValidationError(format!("Failed to parse JSON: {}", e)))?;

        // Extract data using JSON path
        let data = if data_source.json_path == "$" {
            // Root path, use the entire JSON
            json_value
        } else {
            // Use JSON path to extract data
            let json_path = data_source.json_path.as_str();
            let mut selector = jsonpath_lib::selector(&json_value);
            let result = selector(json_path)
                .map_err(|e| Error::ValidationError(format!("Invalid JSON path: {}", e)))?;

            if result.is_empty() {
                return Err(Error::ValidationError(format!(
                    "JSON path '{}' returned no results",
                    json_path
                )));
            }

            result[0].clone()
        };

        // Convert JSON data to HashMap<String, String>
        let data_array = match data {
            Value::Array(arr) => arr,
            _ => {
                return Err(Error::ValidationError(
                    "JSON data must be an array".to_string(),
                ))
            }
        };

        let mut data_rows = Vec::new();

        for item in data_array {
            match item {
                Value::Object(obj) => {
                    let mut row = HashMap::new();
                    for (key, value) in obj {
                        row.insert(key, Self::value_to_string(&value));
                    }
                    data_rows.push(row);
                }
                _ => warn!("Skipping non-object item in JSON data"),
            }
        }

        Ok(data_rows)
    }

    /// Load data from inline data
    fn load_inline_data(data_source: &DataSource) -> Result<Vec<HashMap<String, String>>> {
        let data = data_source.data.as_ref().ok_or_else(|| {
            Error::ValidationError("Data is required for inline data source".to_string())
        })?;

        let mut data_rows = Vec::new();

        for item in data {
            let mut row = HashMap::new();
            for (key, value) in item {
                row.insert(key.clone(), Self::value_to_string(value));
            }
            data_rows.push(row);
        }

        Ok(data_rows)
    }

    /// Convert a JSON value to a string
    fn value_to_string(value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Object(_) | Value::Array(_) => {
                serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
            }
        }
    }

    /// Get the number of data rows
    pub fn row_count(&self) -> usize {
        self.data_rows.len()
    }

    /// Get the maximum number of iterations to run
    pub fn max_iterations(&self) -> usize {
        self.config.max_iterations.unwrap_or(self.data_rows.len())
    }

    /// Get whether to stop on first failure
    pub fn stop_on_failure(&self) -> bool {
        self.config.stop_on_failure
    }

    /// Get the data rows
    pub fn data_rows(&self) -> &Vec<HashMap<String, String>> {
        &self.data_rows
    }

    /// Apply data-driven replacements to a test configuration
    pub fn apply_replacements<T>(&self, config: &T, row: &HashMap<String, String>) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        // Convert config to JSON
        let mut config_json = serde_json::to_value(config)?;

        // Apply replacements
        Self::replace_placeholders(&mut config_json, row);

        // Convert back to config
        let new_config = serde_json::from_value(config_json)?;

        Ok(new_config)
    }

    /// Replace placeholders in a JSON value
    fn replace_placeholders(value: &mut Value, data: &HashMap<String, String>) {
        match value {
            Value::String(s) => {
                let mut result = s.clone();
                for (key, val) in data {
                    let placeholder = format!("{{{{{}}}}}", key);
                    result = result.replace(&placeholder, val);
                }
                *s = result;
            }
            Value::Object(obj) => {
                for (_, v) in obj {
                    Self::replace_placeholders(v, data);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    Self::replace_placeholders(v, data);
                }
            }
            _ => {}
        }
    }
}
