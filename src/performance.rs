use crate::common::{TestConfig, TestResult, TestRunner};
use crate::error::{Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceTestConfig {
    #[serde(flatten)]
    pub base: TestConfig,
    pub target_url: String,
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
    #[serde(default = "default_success_threshold")]
    pub success_threshold: f64,
    #[serde(default = "default_ramp_up_time")]
    pub ramp_up_time_secs: u64,
}

fn default_success_threshold() -> f64 {
    95.0 // 95% success rate threshold by default
}

fn default_ramp_up_time() -> u64 {
    30 // 30 seconds ramp-up time by default
}

#[derive(Debug)]
struct RequestResult {
    status: u16,
    duration: f64,
}

pub struct PerformanceTestRunner {
    client: Client,
    concurrent_users: u32,
    duration_secs: u64,
}

impl PerformanceTestRunner {
    pub fn new(concurrent_users: u32, duration_secs: u64) -> Self {
        Self {
            client: Client::new(),
            concurrent_users,
            duration_secs,
        }
    }

    async fn run_load_test(&self, config: &PerformanceTestConfig) -> Vec<RequestResult> {
        let mut results = Vec::new();
        let start = Instant::now();
        let end = start + Duration::from_secs(self.duration_secs);

        while Instant::now() < end {
            let mut handles = Vec::new();
            
            for _ in 0..self.concurrent_users {
                let client = self.client.clone();
                let target_url = config.target_url.clone();
                let method = config.method.clone();
                let headers = config.headers.clone();
                let body = config.body.clone();
                
                handles.push(tokio::spawn(async move {
                    let request_start = Instant::now();
                    let method = Method::from_bytes(method.as_bytes())
                        .map_err(|e| Error::ValidationError(format!("Invalid HTTP method: {}", e)))?;
                    
                    let mut request = client.request(method, &target_url);
                    
                    if let Some(headers) = headers {
                        for (key, value) in headers.as_object().unwrap() {
                            request = request.header(key, value.as_str().unwrap());
                        }
                    }
                    
                    if let Some(body) = body {
                        request = request.json(&body);
                    }
                    
                    let response = request.send().await?;
                    let duration = request_start.elapsed().as_secs_f64();
                    
                    Ok::<RequestResult, Error>(RequestResult {
                        status: response.status().as_u16(),
                        duration,
                    })
                }));
            }
            
            for handle in handles {
                if let Ok(Ok(result)) = handle.await {
                    results.push(result);
                }
            }
            
            sleep(Duration::from_millis(100)).await;
        }
        
        results
    }
}

#[async_trait]
impl TestRunner for PerformanceTestRunner {
    async fn run(&self, config: &(impl serde::Serialize + Send + Sync)) -> Result<TestResult> {
        let config = serde_json::from_value::<PerformanceTestConfig>(serde_json::to_value(config)?)?;
        let start = Instant::now();
        
        let results = self.run_load_test(&config).await;
        let duration = start.elapsed().as_secs_f64();
        
        let (success_count, error_count) = results.iter().fold((0, 0), |(s, e), r| {
            match r.status {
                200..=299 => (s + 1, e),
                _ => (s, e + 1),
            }
        });
        
        let success_rate = (success_count as f64 / results.len() as f64) * 100.0;
        let status = if success_rate >= config.success_threshold {
            "passed".to_string()
        } else {
            "failed".to_string()
        };
        
        Ok(TestResult {
            name: config.base.name,
            status,
            duration,
            details: Some(serde_json::json!({
                "total_requests": results.len(),
                "success_count": success_count,
                "error_count": error_count,
                "success_rate": success_rate,
                "average_response_time": results.iter().map(|r| r.duration).sum::<f64>() / results.len() as f64,
                "min_response_time": results.iter().map(|r| r.duration).fold(f64::INFINITY, f64::min),
                "max_response_time": results.iter().map(|r| r.duration).fold(f64::NEG_INFINITY, f64::max)
            })),
            timestamp: Utc::now().to_rfc3339(),
        })
    }
} 