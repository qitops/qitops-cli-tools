# Web Testing

QitOps provides powerful web testing capabilities that allow you to automate browser interactions, validate web applications, and ensure your websites function correctly across different scenarios.

## Overview

Web testing in QitOps uses headless browser automation to interact with web pages, perform actions, and validate expected behaviors. This approach allows you to:

- Test complete user journeys through your web application
- Validate UI elements and their properties
- Capture screenshots for visual verification
- Test responsive design across different viewport sizes
- Verify page content, titles, and URLs
- Automate form submissions and user interactions

## Getting Started

### Basic Usage

```bash
# Run a web test
qitops web -c tests/configs/web_test.json

# Run in headless mode (default)
qitops web -c tests/configs/web_test.json --headless

# Run with browser visible
qitops web -c tests/configs/web_test.json --no-headless

# Run with custom viewport
qitops web -c tests/configs/web_test.json --width 1920 --height 1080
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-c, --config <FILE>` | Web test configuration file |
| `--headless` | Run in headless mode (default: true) |
| `--no-headless` | Run with browser visible |
| `--width <PIXELS>` | Viewport width in pixels |
| `--height <PIXELS>` | Viewport height in pixels |
| `--device <DEVICE>` | Emulate a specific device (e.g., "iPhone 12", "iPad") |
| `--timeout <SECONDS>` | Default timeout for actions and assertions |
| `--screenshots` | Capture screenshots during test execution |
| `--screenshots-dir <DIR>` | Directory to save screenshots |
| `-r, --report <FORMAT>` | Report format (json, html, xml, csv) |
| `-o, --output <FILE>` | Output file for the report |

## Configuration

Web tests are defined in JSON configuration files that specify the target URL, actions to perform, and assertions to validate.

### Basic Configuration Structure

```json
{
  "name": "Sample Web Test",
  "description": "Testing a public website",
  "timeout": 30,
  "retries": 3,
  "target_url": "https://example.com",
  "viewport": {
    "width": 1280,
    "height": 800,
    "device_scale_factor": 1.0,
    "is_mobile": false
  },
  "wait_for_selector": "body",
  "wait_timeout_secs": 10,
  "screenshots": true,
  "user_agent": "QitOps-WebTester/1.0",
  "assertions": [
    {
      "assertion_type": "title",
      "expected_value": "Example Domain",
      "comparison": "contains"
    },
    {
      "assertion_type": "element",
      "selector": "h1",
      "expected_value": "true"
    }
  ],
  "actions": [
    {
      "action_type": "wait",
      "wait_time_ms": 1000
    },
    {
      "action_type": "click",
      "selector": "a"
    }
  ]
}
```

### Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | Yes | Name of the test |
| description | string | No | Description of the test |
| timeout | number | No | Default timeout in seconds (default: 30) |
| retries | number | No | Number of retries (default: 3) |
| target_url | string | Yes | URL to test |
| viewport | object | No | Viewport configuration |
| wait_for_selector | string | No | Selector to wait for before starting test |
| wait_timeout_secs | number | No | Timeout for initial wait (default: 10) |
| screenshots | boolean | No | Whether to capture screenshots (default: false) |
| user_agent | string | No | Custom user agent string |
| assertions | array | No | List of assertions to validate |
| actions | array | No | List of actions to perform |

### Viewport Configuration

```json
"viewport": {
  "width": 1280,
  "height": 800,
  "device_scale_factor": 1.0,
  "is_mobile": false
}
```

| Field | Type | Description |
|-------|------|-------------|
| width | number | Viewport width in pixels |
| height | number | Viewport height in pixels |
| device_scale_factor | number | Device scale factor (1.0 for desktop, 2.0+ for high-DPI) |
| is_mobile | boolean | Whether to emulate a mobile device |

## Actions

Actions define the steps to perform during the test, such as clicking elements, typing text, or navigating to different pages.

### Available Actions

#### Click

```json
{
  "action_type": "click",
  "selector": "button.submit",
  "timeout_ms": 5000,
  "button": "left",
  "click_count": 1,
  "delay_ms": 0
}
```

#### Type

```json
{
  "action_type": "type",
  "selector": "input[name='username']",
  "text": "testuser",
  "delay_ms": 50
}
```

#### Navigate

```json
{
  "action_type": "navigate",
  "url": "https://example.com/login",
  "wait_until": "networkidle0"
}
```

#### Wait

```json
{
  "action_type": "wait",
  "wait_time_ms": 1000
}
```

#### Wait For Selector

```json
{
  "action_type": "wait_for_selector",
  "selector": "#content",
  "timeout_ms": 5000,
  "visible": true
}
```

#### Select Option

```json
{
  "action_type": "select",
  "selector": "select#country",
  "value": "US"
}
```

#### Check/Uncheck

```json
{
  "action_type": "check",
  "selector": "input[type='checkbox']",
  "checked": true
}
```

#### Screenshot

```json
{
  "action_type": "screenshot",
  "name": "login-page",
  "full_page": true
}
```

#### Scroll

```json
{
  "action_type": "scroll",
  "x": 0,
  "y": 500
}
```

#### Hover

```json
{
  "action_type": "hover",
  "selector": ".dropdown-menu"
}
```

## Assertions

Assertions validate that the web page meets expected conditions.

### Available Assertions

#### Title

```json
{
  "assertion_type": "title",
  "expected_value": "Dashboard",
  "comparison": "equals"
}
```

#### URL

```json
{
  "assertion_type": "url",
  "expected_value": "https://example.com/dashboard",
  "comparison": "contains"
}
```

#### Element

```json
{
  "assertion_type": "element",
  "selector": ".user-profile",
  "expected_value": "true"
}
```

#### Text Content

```json
{
  "assertion_type": "text",
  "selector": "h1",
  "expected_value": "Welcome to Dashboard",
  "comparison": "equals"
}
```

#### Attribute

```json
{
  "assertion_type": "attribute",
  "selector": "img.logo",
  "attribute": "src",
  "expected_value": "/images/logo.png",
  "comparison": "contains"
}
```

#### Count

```json
{
  "assertion_type": "count",
  "selector": "table tr",
  "expected_value": 5,
  "comparison": "greater_than"
}
```

#### CSS Property

```json
{
  "assertion_type": "css",
  "selector": ".error-message",
  "property": "color",
  "expected_value": "rgb(255, 0, 0)",
  "comparison": "equals"
}
```

### Comparison Types

| Type | Description |
|------|-------------|
| equals | Exact match |
| contains | String contains |
| starts_with | String starts with |
| ends_with | String ends with |
| matches | Regex match |
| greater_than | Numeric greater than |
| less_than | Numeric less than |
| greater_than_or_equal | Numeric greater than or equal |
| less_than_or_equal | Numeric less than or equal |

## Examples

### Login Form Test

```json
{
  "name": "Login Form Test",
  "description": "Test the login functionality",
  "target_url": "https://example.com/login",
  "viewport": {
    "width": 1280,
    "height": 800
  },
  "wait_for_selector": "form",
  "screenshots": true,
  "actions": [
    {
      "action_type": "type",
      "selector": "input[name='username']",
      "text": "testuser"
    },
    {
      "action_type": "type",
      "selector": "input[name='password']",
      "text": "password123"
    },
    {
      "action_type": "screenshot",
      "name": "before-login"
    },
    {
      "action_type": "click",
      "selector": "button[type='submit']"
    },
    {
      "action_type": "wait_for_selector",
      "selector": ".dashboard",
      "timeout_ms": 5000
    },
    {
      "action_type": "screenshot",
      "name": "after-login"
    }
  ],
  "assertions": [
    {
      "assertion_type": "url",
      "expected_value": "/dashboard",
      "comparison": "contains"
    },
    {
      "assertion_type": "element",
      "selector": ".user-profile",
      "expected_value": "true"
    },
    {
      "assertion_type": "text",
      "selector": ".welcome-message",
      "expected_value": "Welcome, Test User",
      "comparison": "contains"
    }
  ]
}
```

### Responsive Design Test

```json
{
  "name": "Responsive Design Test",
  "description": "Test responsive behavior across different devices",
  "target_url": "https://example.com",
  "screenshots": true,
  "actions": [
    {
      "action_type": "set_viewport",
      "width": 1920,
      "height": 1080,
      "device_scale_factor": 1.0,
      "is_mobile": false
    },
    {
      "action_type": "screenshot",
      "name": "desktop-view"
    },
    {
      "action_type": "set_viewport",
      "width": 768,
      "height": 1024,
      "device_scale_factor": 2.0,
      "is_mobile": true
    },
    {
      "action_type": "screenshot",
      "name": "tablet-view"
    },
    {
      "action_type": "set_viewport",
      "width": 375,
      "height": 812,
      "device_scale_factor": 3.0,
      "is_mobile": true
    },
    {
      "action_type": "screenshot",
      "name": "mobile-view"
    }
  ],
  "assertions": [
    {
      "assertion_type": "css",
      "selector": ".mobile-menu",
      "property": "display",
      "expected_value": "block",
      "comparison": "equals"
    }
  ]
}
```

## Best Practices

### Selector Strategy

- Use stable selectors that are less likely to change
- Prefer IDs and data attributes over classes or XPaths
- Add test-specific attributes like `data-testid="login-button"` to your application
- Use specific selectors to avoid ambiguity

### Test Organization

- Group related tests in separate configuration files
- Use descriptive names for tests and screenshots
- Structure tests to follow user journeys
- Keep tests focused on specific functionality

### Performance Considerations

- Use headless mode for CI/CD environments
- Minimize unnecessary actions and waits
- Use appropriate timeouts for your application
- Consider network conditions when testing

### Error Handling

- Add appropriate waits before interactions
- Use retries for flaky operations
- Capture screenshots at failure points
- Add detailed assertions to pinpoint issues

## Integration with CI/CD

Web tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow step
- name: Run Web Tests
  run: |
    qitops web -c tests/configs/web_test.json --headless --screenshots --report html --output web-test-report.html
    
- name: Upload Test Report
  uses: actions/upload-artifact@v2
  with:
    name: web-test-report
    path: web-test-report.html
    
- name: Upload Screenshots
  uses: actions/upload-artifact@v2
  with:
    name: web-test-screenshots
    path: screenshots/
```
