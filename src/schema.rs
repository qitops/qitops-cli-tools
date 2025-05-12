use crate::error::{Error, Result};
use jsonschema::{Draft, JSONSchema};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;

// Define schemas for each test type
static API_TEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "url", "method"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "timeout": {"type": "integer", "minimum": 1},
            "retries": {"type": "integer", "minimum": 0},
            "environment": {"type": "string"},
            "url": {"type": "string", "format": "uri"},
            "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]},
            "headers": {"type": "object"},
            "body": {},
            "expected_status": {"type": "integer", "minimum": 100, "maximum": 599},
            "expected_body": {},
            "max_response_time": {"type": "integer", "minimum": 1},
            "expected_headers": {"type": "object"},
            "json_schema": {},
            "retry": {
                "type": "object",
                "properties": {
                    "max_retries": {"type": "integer", "minimum": 0},
                    "initial_delay_ms": {"type": "integer", "minimum": 1},
                    "max_delay_ms": {"type": "integer", "minimum": 1},
                    "retry_status_codes": {
                        "type": "array",
                        "items": {"type": "integer", "minimum": 100, "maximum": 599}
                    },
                    "retry_on_timeout": {"type": "boolean"},
                    "retry_on_connection_error": {"type": "boolean"}
                }
            }
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid API test schema")
});

static PERFORMANCE_TEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "target_url", "method"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "timeout": {"type": "integer", "minimum": 1},
            "retries": {"type": "integer", "minimum": 0},
            "environment": {"type": "string"},
            "target_url": {"type": "string", "format": "uri"},
            "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]},
            "headers": {"type": "object"},
            "body": {},
            "concurrent_users": {"type": "integer", "minimum": 1},
            "duration": {"type": "integer", "minimum": 1},
            "ramp_up": {"type": "integer", "minimum": 0},
            "success_threshold": {"type": "number", "minimum": 0, "maximum": 100},
            "requests_per_second": {"type": "integer", "minimum": 1}
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid performance test schema")
});

static SECURITY_TEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "target_url"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "timeout": {"type": "integer", "minimum": 1},
            "retries": {"type": "integer", "minimum": 0},
            "environment": {"type": "string"},
            "target_url": {"type": "string", "format": "uri"},
            "headers": {"type": "object"},
            "auth": {
                "type": "object",
                "properties": {
                    "type": {"type": "string", "enum": ["basic", "bearer", "api_key"]},
                    "username": {"type": "string"},
                    "password": {"type": "string"},
                    "token": {"type": "string"},
                    "key_name": {"type": "string"},
                    "key_value": {"type": "string"}
                },
                "required": ["type"]
            },
            "scan_types": {
                "type": "array",
                "items": {"type": "string", "enum": ["headers", "ssl", "vulnerabilities", "sensitive-data"]}
            },
            "max_high_severity_findings": {"type": "integer", "minimum": 0}
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid security test schema")
});

static WEB_TEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "target_url"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "timeout": {"type": "integer", "minimum": 1},
            "retries": {"type": "integer", "minimum": 0},
            "environment": {"type": "string"},
            "target_url": {"type": "string", "format": "uri"},
            "viewport": {
                "type": "object",
                "properties": {
                    "width": {"type": "integer", "minimum": 1},
                    "height": {"type": "integer", "minimum": 1},
                    "device_scale_factor": {"type": "number", "minimum": 0.1},
                    "is_mobile": {"type": "boolean"}
                },
                "required": ["width", "height"]
            },
            "wait_for_selector": {"type": "string"},
            "wait_timeout_secs": {"type": "integer", "minimum": 1},
            "screenshots": {"type": "boolean"},
            "user_agent": {"type": "string"},
            "assertions": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["assertion_type", "expected_value"],
                    "properties": {
                        "assertion_type": {"type": "string", "enum": ["title", "url", "element", "text", "attribute"]},
                        "selector": {"type": "string"},
                        "attribute": {"type": "string"},
                        "expected_value": {"type": "string"},
                        "comparison": {"type": "string", "enum": ["equals", "contains", "startsWith", "endsWith", "matches"]}
                    }
                }
            },
            "actions": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["action_type"],
                    "properties": {
                        "action_type": {"type": "string", "enum": ["click", "type", "wait", "navigate", "select"]},
                        "selector": {"type": "string"},
                        "value": {"type": "string"},
                        "wait_time_ms": {"type": "integer", "minimum": 1}
                    }
                }
            }
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid web test schema")
});

static API_COLLECTION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "requests"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "version": {"type": "string"},
            "variables": {"type": "object"},
            "auth": {
                "type": "object",
                "properties": {
                    "type": {"type": "string", "enum": ["basic", "bearer", "api_key"]},
                    "username": {"type": "string"},
                    "password": {"type": "string"},
                    "token": {"type": "string"},
                    "key_name": {"type": "string"},
                    "key_value": {"type": "string"}
                },
                "required": ["type"]
            },
            "defaults": {
                "type": "object",
                "properties": {
                    "headers": {"type": "object"},
                    "timeout": {"type": "integer", "minimum": 1},
                    "retries": {"type": "integer", "minimum": 0}
                }
            },
            "requests": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["name", "url", "method"],
                    "properties": {
                        "name": {"type": "string"},
                        "description": {"type": "string"},
                        "id": {"type": "string"},
                        "url": {"type": "string"},
                        "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]},
                        "headers": {"type": "object"},
                        "body": {},
                        "expected_status": {"type": "integer", "minimum": 100, "maximum": 599},
                        "expected_body": {},
                        "expected_body_type": {"type": "string", "enum": ["object", "array", "string", "number", "boolean", "null"]},
                        "depends_on": {
                            "type": "array",
                            "items": {"type": "string"}
                        },
                        "capture": {"type": "object"}
                    }
                }
            },
            "environments": {"type": "object"},
            "run_options": {
                "type": "object",
                "properties": {
                    "sequential": {"type": "boolean"},
                    "stop_on_failure": {"type": "boolean"},
                    "delay_between_requests_ms": {"type": "integer", "minimum": 0}
                }
            }
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid API collection schema")
});

static ENHANCED_PERFORMANCE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["name", "load_profile", "scenarios"],
        "properties": {
            "name": {"type": "string"},
            "description": {"type": "string"},
            "timeout": {"type": "integer", "minimum": 1},
            "retries": {"type": "integer", "minimum": 0},
            "environment": {"type": "string"},
            "load_profile": {
                "type": "object",
                "required": ["type"],
                "properties": {
                    "type": {"type": "string", "enum": ["constant_vus", "ramping_vus", "constant_arrival_rate", "ramping_arrival_rate", "spike"]},
                    "initial": {"type": "integer", "minimum": 1},
                    "target": {"type": "integer", "minimum": 0},
                    "duration_secs": {"type": "integer", "minimum": 1},
                    "stages": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "required": ["duration_secs", "target"],
                            "properties": {
                                "duration_secs": {"type": "integer", "minimum": 1},
                                "target": {"type": "integer", "minimum": 0}
                            }
                        }
                    }
                }
            },
            "scenarios": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["name", "target_url", "method"],
                    "properties": {
                        "name": {"type": "string"},
                        "target_url": {"type": "string", "format": "uri"},
                        "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]},
                        "headers": {"type": "object"},
                        "body": {},
                        "weight": {"type": "integer", "minimum": 1},
                        "tags": {"type": "object"}
                    }
                }
            },
            "thresholds": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["metric", "expression"],
                    "properties": {
                        "metric": {"type": "string"},
                        "expression": {"type": "string"},
                        "abort_on_fail": {"type": "boolean"}
                    }
                }
            },
            "success_threshold": {"type": "number", "minimum": 0, "maximum": 100},
            "custom_metrics": {
                "type": "array",
                "items": {"type": "string"}
            },
            "stream_metrics": {"type": "boolean"},
            "metrics_interval_secs": {"type": "integer", "minimum": 1}
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid enhanced performance test schema")
});

static DATA_DRIVEN_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["data_source"],
        "properties": {
            "data_source": {
                "type": "object",
                "required": ["source_type"],
                "properties": {
                    "source_type": {"type": "string", "enum": ["csv", "json"]},
                    "file_path": {"type": "string"},
                    "delimiter": {"type": "string"},
                    "has_header": {"type": "boolean"},
                    "json_path": {"type": "string"},
                    "data": {"type": "array"}
                }
            },
            "stop_on_failure": {"type": "boolean"},
            "max_iterations": {"type": "integer", "minimum": 1},
            "placeholders": {
                "type": "object",
                "additionalProperties": {"type": "string"}
            }
        }
    });

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("Invalid data-driven test schema")
});

// Map of test types to their schemas
static SCHEMAS: Lazy<HashMap<&'static str, &'static JSONSchema>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("api", &*API_TEST_SCHEMA);
    map.insert("performance", &*PERFORMANCE_TEST_SCHEMA);
    map.insert("performance_enhanced", &*ENHANCED_PERFORMANCE_SCHEMA);
    map.insert("security", &*SECURITY_TEST_SCHEMA);
    map.insert("web", &*WEB_TEST_SCHEMA);
    map.insert("api_collection", &*API_COLLECTION_SCHEMA);
    map.insert("data_driven", &*DATA_DRIVEN_SCHEMA);
    map
});

/// Validate a test configuration against its schema
pub fn validate_config(config: &Value, test_type: &str) -> Result<()> {
    let schema = SCHEMAS.get(test_type).ok_or_else(|| {
        Error::ValidationError(format!("Unknown test type: {}. Supported types are: api, performance, performance_enhanced, security, web, api_collection, data_driven", test_type))
    })?;

    let validation = schema.validate(config);
    if let Err(errors) = validation {
        let mut error_messages: Vec<String> = Vec::new();

        for error in errors {
            let path = error.instance_path.to_string();
            let message = error.to_string();

            // Add more context to the error message
            let formatted_error = if path.is_empty() {
                format!("Root: {}", message)
            } else {
                format!("At '{}': {}", path, message)
            };

            error_messages.push(formatted_error);

            // Add suggestions for common errors
            if message.contains("required property") {
                let property = message.split("'").nth(1).unwrap_or("");
                error_messages.push(format!(
                    "  Suggestion: Add the '{}' property to your configuration",
                    property
                ));
            } else if message.contains("enum") {
                let valid_values = message
                    .split("one of: ")
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches('.');
                error_messages.push(format!(
                    "  Suggestion: Use one of the allowed values: {}",
                    valid_values
                ));
            } else if message.contains("type") && message.contains("string") {
                error_messages.push(
                    "  Suggestion: Ensure the value is a string enclosed in quotes".to_string(),
                );
            } else if message.contains("type") && message.contains("integer") {
                error_messages
                    .push("  Suggestion: Ensure the value is a number without quotes".to_string());
            } else if message.contains("type") && message.contains("boolean") {
                error_messages.push("  Suggestion: Use true or false without quotes".to_string());
            }
        }

        return Err(Error::ValidationError(format!(
            "Configuration validation failed for test type '{}':\n{}\n\nPlease check your configuration file against the schema requirements.",
            test_type,
            error_messages.join("\n")
        )));
    }

    Ok(())
}

/// Determine the test type from a configuration file
pub fn determine_test_type(path: &std::path::Path) -> Result<String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| Error::ValidationError("Invalid file path".to_string()))?
        .to_str()
        .ok_or_else(|| Error::ValidationError("Invalid file name".to_string()))?;

    if file_name.contains("api_collection") {
        return Ok("api_collection".to_string());
    } else if file_name.contains("api") {
        return Ok("api".to_string());
    } else if file_name.contains("performance_enhanced") {
        return Ok("performance_enhanced".to_string());
    } else if file_name.contains("performance") {
        return Ok("performance".to_string());
    } else if file_name.contains("security") {
        return Ok("security".to_string());
    } else if file_name.contains("web") {
        return Ok("web".to_string());
    } else if file_name.contains("data_driven") {
        // Try to determine from content
        return Ok("api".to_string()); // Default to API for now
    }

    Err(Error::ValidationError(format!(
        "Could not determine test type from file name: {}",
        file_name
    )))
}
