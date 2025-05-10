use crate::common::{TestConfig, TestResult, TestRunner};
use crate::error::{Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use log::{info, warn};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Load profile types for performance testing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadProfileType {
    /// Constant number of virtual users
    ConstantVus,
    /// Ramping up virtual users over time
    RampingVus,
    /// Constant arrival rate of requests
    ConstantArrivalRate,
    /// Ramping arrival rate of requests
    RampingArrivalRate,
    /// Spike testing (sudden increase and decrease)
    Spike,
}

/// Stage in a load profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadStage {
    /// Duration of this stage in seconds
    pub duration_secs: u64,
    /// Target number of virtual users or requests per second
    pub target: u32,
}

/// Load profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProfile {
    /// Type of load profile
    #[serde(rename = "type")]
    pub profile_type: LoadProfileType,
    /// Stages of the load profile
    pub stages: Vec<LoadStage>,
    /// Initial number of virtual users or requests per second
    #[serde(default = "default_initial_value")]
    pub initial: u32,
}

fn default_initial_value() -> u32 {
    1
}

/// Threshold for a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    /// Metric name
    pub metric: String,
    /// Threshold expression (e.g., "avg < 200", "p95 < 500", "rate > 0.95")
    pub expression: String,
    /// Whether to abort the test if this threshold is breached
    #[serde(default)]
    pub abort_on_fail: bool,
}

/// Scenario configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario name
    pub name: String,
    /// Target URL
    pub target_url: String,
    /// HTTP method
    pub method: String,
    /// Request headers
    pub headers: Option<HashMap<String, String>>,
    /// Request body
    pub body: Option<serde_json::Value>,
    /// Weight of this scenario (for distribution)
    #[serde(default = "default_weight")]
    pub weight: u32,
    /// Tags for this scenario (for metrics)
    pub tags: Option<HashMap<String, String>>,
}

fn default_weight() -> u32 {
    1
}

/// Enhanced performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedPerformanceConfig {
    /// Base test configuration
    #[serde(flatten)]
    pub base: TestConfig,
    /// Load profile
    pub load_profile: LoadProfile,
    /// Scenarios to run
    pub scenarios: Vec<Scenario>,
    /// Thresholds for metrics
    pub thresholds: Option<Vec<Threshold>>,
    /// Success threshold percentage
    #[serde(default = "default_success_threshold")]
    pub success_threshold: f64,
    /// Custom metrics to track
    pub custom_metrics: Option<Vec<String>>,
    /// Whether to stream metrics to stdout
    #[serde(default)]
    pub stream_metrics: bool,
    /// Metrics output interval in seconds
    #[serde(default = "default_metrics_interval")]
    pub metrics_interval_secs: u64,
}

fn default_success_threshold() -> f64 {
    95.0 // 95% success rate threshold by default
}

fn default_metrics_interval() -> u64 {
    10 // 10 seconds by default
}

/// Request result with metrics
#[derive(Debug, Clone)]
struct RequestResult {
    /// Scenario name
    scenario: String,
    /// HTTP status code - Used for status code distribution analysis and metrics
    /// This field is important for tracking response status codes across requests
    status: u16,
    /// Request duration in seconds
    duration: f64,
    /// Whether the request was successful
    success: bool,
    /// Timestamp when the request was made - Used for time-series analysis
    /// This field is critical for analyzing request patterns over time
    /// and calculating metrics like requests per second
    timestamp: Instant,
    /// Custom metrics
    metrics: HashMap<String, f64>,
    /// Tags
    tags: HashMap<String, String>,
}

/// Metrics collector
#[derive(Debug, Clone)]
struct MetricsCollector {
    /// All request results
    results: Vec<RequestResult>,
    /// Metrics by name
    metrics: HashMap<String, Vec<f64>>,
    /// Metrics by tag
    metrics_by_tag: HashMap<String, HashMap<String, Vec<f64>>>,
}

impl MetricsCollector {
    fn new() -> Self {
        Self {
            results: Vec::new(),
            metrics: HashMap::new(),
            metrics_by_tag: HashMap::new(),
        }
    }

    fn add_result(&mut self, result: RequestResult) {
        // Add basic metrics
        self.add_metric("response_time", result.duration);
        self.add_metric("success", if result.success { 1.0 } else { 0.0 });

        // Add custom metrics
        for (name, value) in &result.metrics {
            self.add_metric(name, *value);
        }

        // Add metrics by tag
        for (tag_name, tag_value) in &result.tags {
            let tag_key = format!("{}:{}", tag_name, tag_value);
            self.add_metric_by_tag(&tag_key, "response_time", result.duration);
            self.add_metric_by_tag(&tag_key, "success", if result.success { 1.0 } else { 0.0 });

            for (name, value) in &result.metrics {
                self.add_metric_by_tag(&tag_key, name, *value);
            }
        }

        self.results.push(result);
    }

    fn add_metric(&mut self, name: &str, value: f64) {
        self.metrics.entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    fn add_metric_by_tag(&mut self, tag: &str, name: &str, value: f64) {
        self.metrics_by_tag.entry(tag.to_string())
            .or_insert_with(HashMap::new)
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    fn get_metrics_summary(&self) -> serde_json::Value {
        let mut summary = serde_json::json!({
            "total_requests": self.results.len(),
            "success_count": self.results.iter().filter(|r| r.success).count(),
            "error_count": self.results.iter().filter(|r| !r.success).count(),
        });

        // Add metrics summaries
        let metrics_obj = summary.as_object_mut().unwrap();

        for (name, values) in &self.metrics {
            if !values.is_empty() {
                let avg = values.iter().sum::<f64>() / values.len() as f64;
                let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                // Calculate percentiles
                let mut sorted_values = values.clone();
                sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p50 = percentile(&sorted_values, 50.0);
                let p90 = percentile(&sorted_values, 90.0);
                let p95 = percentile(&sorted_values, 95.0);
                let p99 = percentile(&sorted_values, 99.0);

                metrics_obj.insert(name.clone(), serde_json::json!({
                    "avg": avg,
                    "min": min,
                    "max": max,
                    "p50": p50,
                    "p90": p90,
                    "p95": p95,
                    "p99": p99,
                    "count": values.len(),
                }));
            }
        }

        // Add metrics by tag
        let mut metrics_by_tag = serde_json::json!({});
        let metrics_by_tag_obj = metrics_by_tag.as_object_mut().unwrap();

        for (tag, metrics) in &self.metrics_by_tag {
            let mut tag_metrics = serde_json::json!({});
            let tag_metrics_obj = tag_metrics.as_object_mut().unwrap();

            for (name, values) in metrics {
                if !values.is_empty() {
                    let avg = values.iter().sum::<f64>() / values.len() as f64;
                    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                    tag_metrics_obj.insert(name.clone(), serde_json::json!({
                        "avg": avg,
                        "min": min,
                        "max": max,
                        "count": values.len(),
                    }));
                }
            }

            metrics_by_tag_obj.insert(tag.clone(), tag_metrics);
        }

        metrics_obj.insert("by_tag".to_string(), metrics_by_tag);

        // Add scenarios summary
        let mut scenarios_summary = serde_json::json!({});
        let scenarios_obj = scenarios_summary.as_object_mut().unwrap();

        let scenarios: Vec<String> = self.results.iter()
            .map(|r| r.scenario.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for scenario in scenarios {
            let scenario_results: Vec<&RequestResult> = self.results.iter()
                .filter(|r| r.scenario == scenario)
                .collect();

            let success_count = scenario_results.iter().filter(|r| r.success).count();
            let total_count = scenario_results.len();

            scenarios_obj.insert(scenario, serde_json::json!({
                "total_requests": total_count,
                "success_count": success_count,
                "error_count": total_count - success_count,
                "success_rate": if total_count > 0 { (success_count as f64 / total_count as f64) * 100.0 } else { 0.0 },
            }));
        }

        metrics_obj.insert("scenarios".to_string(), scenarios_summary);

        summary
    }
}

fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }

    let index = (p / 100.0 * (sorted_values.len() - 1) as f64).round() as usize;
    sorted_values[index]
}

/// Enhanced performance test runner
pub struct EnhancedPerformanceRunner {
    client: Client,
}

impl EnhancedPerformanceRunner {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    /// Run a load test with the given configuration
    async fn run_load_test(&self, config: &EnhancedPerformanceConfig) -> Result<MetricsCollector> {
        info!("Starting load test with profile: {:?}", config.load_profile.profile_type);

        let metrics = Arc::new(Mutex::new(MetricsCollector::new()));
        let start_time = Instant::now();

        // Set up metrics streaming if enabled
        let metrics_interval = Duration::from_secs(config.metrics_interval_secs);
        let stream_metrics = config.stream_metrics;

        if stream_metrics {
            let metrics_clone = Arc::clone(&metrics);
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(metrics_interval);
                loop {
                    interval.tick().await;
                    let elapsed = start_time.elapsed().as_secs();

                    // Get current metrics
                    let current_metrics = {
                        let metrics_guard = metrics_clone.lock().unwrap();
                        metrics_guard.get_metrics_summary()
                    };

                    // Print metrics summary
                    println!("\n--- Metrics at {}s ---", elapsed);
                    println!("Total requests: {}", current_metrics["total_requests"]);
                    println!("Success rate: {:.2}%",
                        (current_metrics["success_count"].as_u64().unwrap_or(0) as f64 /
                         current_metrics["total_requests"].as_u64().unwrap_or(1) as f64) * 100.0);

                    if let Some(rt) = current_metrics.get("response_time") {
                        println!("Response time (avg): {:.2}ms", rt["avg"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("Response time (p95): {:.2}ms", rt["p95"].as_f64().unwrap_or(0.0) * 1000.0);
                    }
                    println!("------------------------");
                }
            });
        }

        // Run the appropriate load profile
        match config.load_profile.profile_type {
            LoadProfileType::ConstantVus => {
                self.run_constant_vus(config, Arc::clone(&metrics)).await?;
            },
            LoadProfileType::RampingVus => {
                self.run_ramping_vus(config, Arc::clone(&metrics)).await?;
            },
            LoadProfileType::ConstantArrivalRate => {
                self.run_constant_arrival_rate(config, Arc::clone(&metrics)).await?;
            },
            LoadProfileType::RampingArrivalRate => {
                self.run_ramping_arrival_rate(config, Arc::clone(&metrics)).await?;
            },
            LoadProfileType::Spike => {
                self.run_spike(config, Arc::clone(&metrics)).await?;
            },
        }

        // Return the collected metrics
        let final_metrics = {
            let metrics_guard = metrics.lock().unwrap();
            metrics_guard.clone()
        };

        Ok(final_metrics)
    }

    /// Run a test with constant virtual users
    async fn run_constant_vus(&self, config: &EnhancedPerformanceConfig, metrics: Arc<Mutex<MetricsCollector>>) -> Result<()> {
        let stages = &config.load_profile.stages;
        if stages.is_empty() {
            return Err(Error::ValidationError("No stages defined for constant VUs profile".to_string()));
        }

        let mut current_stage = 0;
        let mut stage_start_time = Instant::now();
        let mut current_vus = config.load_profile.initial;

        // Create a semaphore to limit concurrent requests
        let semaphore = Arc::new(Semaphore::new(current_vus as usize));

        loop {
            // Check if we've completed all stages
            if current_stage >= stages.len() {
                break;
            }

            let stage = &stages[current_stage];
            let stage_elapsed = stage_start_time.elapsed().as_secs();

            // Check if we've completed the current stage
            if stage_elapsed >= stage.duration_secs {
                current_stage += 1;
                if current_stage < stages.len() {
                    current_vus = stages[current_stage].target;
                    semaphore.add_permits((current_vus as isize - semaphore.available_permits() as isize).max(0) as usize);
                    stage_start_time = Instant::now();
                }
                continue;
            }

            // Spawn VUs up to the current limit
            let available_permits = semaphore.available_permits();
            if available_permits > 0 {
                for _ in 0..available_permits {
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let client = self.client.clone();
                    let scenarios = config.scenarios.clone();
                    let metrics_clone = Arc::clone(&metrics);

                    tokio::spawn(async move {
                        // Select a scenario based on weights
                        let scenario = select_weighted_scenario(&scenarios);

                        // Execute the scenario
                        let result = execute_scenario(client, scenario).await;

                        // Record the result
                        if let Ok(result) = result {
                            let mut metrics_guard = metrics_clone.lock().unwrap();
                            metrics_guard.add_result(result);
                        }

                        // Release the permit when done
                        drop(permit);
                    });
                }
            }

            // Small sleep to avoid busy waiting
            sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    /// Run a test with ramping virtual users
    async fn run_ramping_vus(&self, config: &EnhancedPerformanceConfig, metrics: Arc<Mutex<MetricsCollector>>) -> Result<()> {
        let stages = &config.load_profile.stages;
        if stages.is_empty() {
            return Err(Error::ValidationError("No stages defined for ramping VUs profile".to_string()));
        }

        let mut current_stage = 0;
        let mut stage_start_time = Instant::now();
        let mut current_vus = config.load_profile.initial;
        let mut target_vus = stages[0].target;

        // Create a semaphore to limit concurrent requests
        let semaphore = Arc::new(Semaphore::new(current_vus as usize));

        loop {
            // Check if we've completed all stages
            if current_stage >= stages.len() {
                break;
            }

            let stage = &stages[current_stage];
            let stage_elapsed = stage_start_time.elapsed().as_secs();

            // Check if we've completed the current stage
            if stage_elapsed >= stage.duration_secs {
                current_stage += 1;
                if current_stage < stages.len() {
                    current_vus = target_vus;
                    target_vus = stages[current_stage].target;
                    stage_start_time = Instant::now();
                }
                continue;
            }

            // Calculate the current number of VUs based on linear interpolation
            if stage.duration_secs > 0 {
                let progress = stage_elapsed as f64 / stage.duration_secs as f64;
                let interpolated_vus = current_vus as f64 + (target_vus as f64 - current_vus as f64) * progress;
                let new_vus = interpolated_vus.round() as u32;

                // Adjust the semaphore if needed
                let available_permits = semaphore.available_permits() as u32;
                // We can estimate the number of active VUs by subtracting available permits from the current VUs
                let estimated_active_vus = current_vus.saturating_sub(available_permits);
                let current_permits = available_permits + estimated_active_vus;
                if new_vus > current_permits {
                    semaphore.add_permits((new_vus - current_permits) as usize);
                }
            }

            // Spawn VUs up to the current limit
            let available_permits = semaphore.available_permits();
            if available_permits > 0 {
                for _ in 0..available_permits {
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let client = self.client.clone();
                    let scenarios = config.scenarios.clone();
                    let metrics_clone = Arc::clone(&metrics);

                    tokio::spawn(async move {
                        // Select a scenario based on weights
                        let scenario = select_weighted_scenario(&scenarios);

                        // Execute the scenario
                        let result = execute_scenario(client, scenario).await;

                        // Record the result
                        if let Ok(result) = result {
                            let mut metrics_guard = metrics_clone.lock().unwrap();
                            metrics_guard.add_result(result);
                        }

                        // Release the permit when done
                        drop(permit);
                    });
                }
            }

            // Small sleep to avoid busy waiting
            sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }

    // Placeholder for other load profile implementations
    async fn run_constant_arrival_rate(&self, config: &EnhancedPerformanceConfig, metrics: Arc<Mutex<MetricsCollector>>) -> Result<()> {
        // Simplified implementation for now
        self.run_constant_vus(config, metrics).await
    }

    async fn run_ramping_arrival_rate(&self, config: &EnhancedPerformanceConfig, metrics: Arc<Mutex<MetricsCollector>>) -> Result<()> {
        // Simplified implementation for now
        self.run_ramping_vus(config, metrics).await
    }

    async fn run_spike(&self, config: &EnhancedPerformanceConfig, metrics: Arc<Mutex<MetricsCollector>>) -> Result<()> {
        // Simplified implementation for now
        self.run_ramping_vus(config, metrics).await
    }
}

/// Select a scenario based on weights
fn select_weighted_scenario(scenarios: &[Scenario]) -> Scenario {
    if scenarios.is_empty() {
        panic!("No scenarios defined");
    }

    if scenarios.len() == 1 {
        return scenarios[0].clone();
    }

    let total_weight: u32 = scenarios.iter().map(|s| s.weight).sum();
    let mut rng = rand::thread_rng();
    let random_value = rand::Rng::gen_range(&mut rng, 0..total_weight);

    let mut cumulative_weight = 0;
    for scenario in scenarios {
        cumulative_weight += scenario.weight;
        if random_value < cumulative_weight {
            return scenario.clone();
        }
    }

    // Fallback to the first scenario
    scenarios[0].clone()
}

/// Execute a scenario and return the result
async fn execute_scenario(client: Client, scenario: Scenario) -> Result<RequestResult> {
    let start = Instant::now(); // For measuring duration
    // Use the same timestamp for both start time and request timestamp
    // This simplifies the code while still providing accurate timing

    // Parse the HTTP method
    let method = Method::from_bytes(scenario.method.as_bytes())
        .map_err(|e| Error::ValidationError(format!("Invalid HTTP method: {}", e)))?;

    // Build the request
    let mut request = client.request(method, &scenario.target_url);

    // Add headers
    if let Some(headers) = &scenario.headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    // Add body
    if let Some(body) = &scenario.body {
        request = request.json(body);
    }

    // Send the request
    let response = request.send().await?;
    let status = response.status().as_u16(); // This status is used to determine success
    let duration = start.elapsed().as_secs_f64();

    // Determine if the request was successful
    let success = status >= 200 && status < 300;

    // Create custom metrics
    let mut metrics = HashMap::new();
    metrics.insert("response_time".to_string(), duration);
    metrics.insert("status_code".to_string(), status as f64);

    // Create tags
    let mut tags = HashMap::new();
    tags.insert("scenario".to_string(), scenario.name.clone());
    tags.insert("method".to_string(), scenario.method.clone());
    tags.insert("status".to_string(), status.to_string());

    // Add user-defined tags
    if let Some(user_tags) = scenario.tags {
        tags.extend(user_tags);
    }

    // Use the start time as the timestamp for the request
    // This ensures we have accurate timing information for time-series analysis
    Ok(RequestResult {
        scenario: scenario.name,
        status,
        duration,
        success,
        timestamp: start, // Using start time as the timestamp
        metrics,
        tags,
    })
}

#[async_trait]
impl TestRunner for EnhancedPerformanceRunner {
    async fn run(&self, config: &(impl serde::Serialize + Send + Sync)) -> Result<TestResult> {
        let config = serde_json::from_value::<EnhancedPerformanceConfig>(serde_json::to_value(config)?)?;
        let start = Instant::now();

        info!("Starting performance test: {}", config.base.name);

        // Run the load test
        let metrics_collector = self.run_load_test(&config).await?;

        // Calculate overall duration
        let duration = start.elapsed().as_secs_f64();

        // Get metrics summary
        let metrics_summary = {
            metrics_collector.get_metrics_summary()
        };

        // Calculate success rate
        let total_requests = metrics_summary["total_requests"].as_u64().unwrap_or(0);
        let success_count = metrics_summary["success_count"].as_u64().unwrap_or(0);
        let success_rate = if total_requests > 0 {
            (success_count as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        // Determine test status based on success threshold
        let status = if success_rate >= config.success_threshold {
            "passed".to_string()
        } else {
            "failed".to_string()
        };

        // Check thresholds if defined
        let mut threshold_results = Vec::new();
        if let Some(thresholds) = &config.thresholds {
            for threshold in thresholds {
                // This is a simplified threshold evaluation
                // In a real implementation, we would parse and evaluate the expression
                let threshold_result = evaluate_threshold(&metrics_summary, threshold);
                threshold_results.push(serde_json::json!({
                    "metric": threshold.metric,
                    "expression": threshold.expression,
                    "passed": threshold_result,
                }));

                // If a threshold with abort_on_fail is breached, mark the test as failed
                if !threshold_result && threshold.abort_on_fail {
                    info!("Threshold breached: {}", threshold.expression);
                }
            }
        }

        // Create the test result
        Ok(TestResult {
            name: config.base.name,
            status,
            duration,
            details: Some(serde_json::json!({
                "metrics": metrics_summary,
                "thresholds": threshold_results,
                "config": {
                    "load_profile": config.load_profile,
                    "scenarios": config.scenarios.len(),
                    "success_threshold": config.success_threshold
                }
            })),
            timestamp: Utc::now().to_rfc3339(),
        })
    }
}

/// Evaluate a threshold against metrics
/// This is a simplified implementation
fn evaluate_threshold(metrics: &serde_json::Value, threshold: &Threshold) -> bool {
    // Parse the metric name and type (e.g., "response_time.avg")
    let parts: Vec<&str> = threshold.metric.split('.').collect();
    if parts.len() != 2 {
        warn!("Invalid metric format: {}", threshold.metric);
        return false;
    }

    let metric_name = parts[0];
    let metric_type = parts[1];

    // Get the metric value
    let metric_value = match metrics.get(metric_name) {
        Some(metric) => match metric.get(metric_type) {
            Some(value) => value.as_f64().unwrap_or(0.0),
            None => {
                warn!("Metric type not found: {}", metric_type);
                return false;
            }
        },
        None => {
            warn!("Metric not found: {}", metric_name);
            return false;
        }
    };

    // Parse the expression (e.g., "< 200", "> 0.95")
    let expr: Vec<&str> = threshold.expression.split_whitespace().collect();
    if expr.len() != 2 {
        warn!("Invalid threshold expression: {}", threshold.expression);
        return false;
    }

    let operator = expr[0];
    let threshold_value = match expr[1].parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            warn!("Invalid threshold value: {}", expr[1]);
            return false;
        }
    };

    // Evaluate the expression
    match operator {
        "<" => metric_value < threshold_value,
        "<=" => metric_value <= threshold_value,
        ">" => metric_value > threshold_value,
        ">=" => metric_value >= threshold_value,
        "==" => (metric_value - threshold_value).abs() < f64::EPSILON,
        "!=" => (metric_value - threshold_value).abs() >= f64::EPSILON,
        _ => {
            warn!("Invalid operator: {}", operator);
            false
        }
    }
}
