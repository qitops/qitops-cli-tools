use crate::ai::model::AiModel;
use crate::error::Result;
use crate::web::WebTestConfig;
use log::{debug, info};

/// Generate a web test configuration using AI
pub async fn generate_web_test(
    model: &dyn AiModel,
    description: &str,
    url: Option<&str>,
) -> Result<WebTestConfig> {
    info!("Generating web test configuration using AI for: {}", description);

    let prompt = format!(
        r#"
You are a QA automation expert. Create a web test configuration for the following description:

Description: {}
{}

The configuration should be a valid JSON object with the following structure:
{{
  "name": "Test name",
  "description": "Test description",
  "target_url": "URL to test",
  "user_agent": "QitOps-WebTester/1.0",
  "wait_timeout_secs": 30,
  "wait_for_selector": "CSS selector to wait for (optional)",
  "viewport": {{
    "width": 1280,
    "height": 720,
    "device_scale_factor": 1.0,
    "is_mobile": false
  }},
  "screenshots": true,
  "assertions": [
    {{
      "assertion_type": "title",
      "expected_value": "Expected page title",
      "comparison": "contains"
    }},
    {{
      "assertion_type": "element",
      "selector": "CSS selector",
      "expected_value": "true"
    }},
    {{
      "assertion_type": "text",
      "expected_value": "Text to find on page",
      "comparison": "contains"
    }},
    {{
      "assertion_type": "attribute",
      "selector": "CSS selector",
      "attribute": "attribute name",
      "expected_value": "expected attribute value",
      "comparison": "equals"
    }}
  ],
  "actions": [
    {{
      "action_type": "click",
      "selector": "CSS selector"
    }},
    {{
      "action_type": "type",
      "selector": "CSS selector",
      "value": "Text to type"
    }},
    {{
      "action_type": "wait",
      "wait_time_ms": 1000
    }},
    {{
      "action_type": "navigate",
      "value": "URL to navigate to"
    }},
    {{
      "action_type": "select",
      "selector": "CSS selector",
      "value": "Option value"
    }},
    {{
      "action_type": "check",
      "selector": "CSS selector"
    }},
    {{
      "action_type": "uncheck",
      "selector": "CSS selector"
    }},
    {{
      "action_type": "wait_for_selector",
      "selector": "CSS selector",
      "wait_time_ms": 5000
    }}
  ]
}}

Create a realistic web test for the given description. Include appropriate assertions and actions based on the description.
Only include the JSON object in your response, nothing else.
"#,
        description,
        if let Some(url) = url {
            format!("URL: {}", url)
        } else {
            "".to_string()
        }
    );

    debug!("Sending prompt to AI model: {}", prompt);
    let response = model.generate(&prompt).await?;
    debug!("Received response from AI model: {}", response.content);

    // Extract JSON from the response
    let json_str = extract_json(&response.content)?;
    let config: WebTestConfig = serde_json::from_str(json_str)?;

    Ok(config)
}

/// Analyze web test results using AI
pub async fn analyze_web_test_results(
    model: &dyn AiModel,
    test_config: &WebTestConfig,
    test_results: &serde_json::Value,
) -> Result<String> {
    info!("Analyzing web test results using AI");

    let prompt = format!(
        r#"
You are a QA automation expert. Analyze the following web test results:

Test Configuration:
{}

Test Results:
{}

Provide a detailed analysis of the test results, including:
1. Overall test status (passed/failed)
2. Analysis of each assertion result
3. Analysis of each action result
4. Recommendations for improving the test
5. Potential issues with the website being tested

Format your response in Markdown.
"#,
        serde_json::to_string_pretty(test_config)?,
        serde_json::to_string_pretty(test_results)?
    );

    debug!("Sending prompt to AI model: {}", prompt);
    let response = model.generate(&prompt).await?;
    debug!("Received response from AI model: {}", response.content);

    Ok(response.content)
}

/// Generate improvement suggestions for web tests using AI
pub async fn generate_web_test_improvements(
    model: &dyn AiModel,
    test_config: &WebTestConfig,
    test_results: &serde_json::Value,
) -> Result<String> {
    info!("Generating web test improvement suggestions using AI");

    let prompt = format!(
        r#"
You are a QA automation expert. Suggest improvements for the following web test:

Test Configuration:
{}

Test Results:
{}

Provide detailed suggestions for improving the test, including:
1. Additional assertions that could be added
2. Additional actions that could be performed
3. Changes to existing assertions or actions
4. Better selectors or waiting strategies
5. Performance improvements
6. Reliability improvements

Format your response in Markdown.
"#,
        serde_json::to_string_pretty(test_config)?,
        serde_json::to_string_pretty(test_results)?
    );

    debug!("Sending prompt to AI model: {}", prompt);
    let response = model.generate(&prompt).await?;
    debug!("Received response from AI model: {}", response.content);

    Ok(response.content)
}

// Helper function to extract JSON from a string
fn extract_json(text: &str) -> Result<&str> {
    // Try to find JSON object in the text
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return Ok(&text[start..=end]);
            }
        }
    }

    // If no JSON object found, return the original text
    Ok(text)
}
