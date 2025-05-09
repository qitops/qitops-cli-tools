use crate::error::{Error, Result};
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::collections::HashMap;
use once_cell::sync::Lazy;

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

// Map of test types to their schemas
static SCHEMAS: Lazy<HashMap<&'static str, &'static JSONSchema>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("api", &*API_TEST_SCHEMA);
    map.insert("performance", &*PERFORMANCE_TEST_SCHEMA);
    map.insert("security", &*SECURITY_TEST_SCHEMA);
    map.insert("web", &*WEB_TEST_SCHEMA);
    map.insert("api_collection", &*API_COLLECTION_SCHEMA);
    map
});

/// Validate a test configuration against its schema
pub fn validate_config(config: &Value, test_type: &str) -> Result<()> {
    let schema = SCHEMAS.get(test_type).ok_or_else(|| {
        Error::ValidationError(format!("Unknown test type: {}", test_type))
    })?;
    
    let validation = schema.validate(config);
    if let Err(errors) = validation {
        let error_messages: Vec<String> = errors
            .map(|e| format!("{}", e))
            .collect();
        return Err(Error::ValidationError(format!(
            "Configuration validation failed:\n{}",
            error_messages.join("\n")
        )));
    }
    
    Ok(())
}
