use clap::{Parser, Subcommand};
use env_logger::Env;
use log::{info, warn};
use std::path::PathBuf;

// Import modules from the crate
use qitops::ai::{AiConfig, AiModelType, AiTestGenerator};
use qitops::api::{ApiTestConfig, ApiTestRunner};
use qitops::api_collection::ApiCollectionRunner;
use qitops::common::{TestRunner, load_config};
use qitops::data_driven::{DataDrivenConfig, DataDrivenRunner};
use qitops::error::{Error, Result};
use qitops::performance::{PerformanceTestConfig, PerformanceTestRunner};
use qitops::performance_enhanced::{EnhancedPerformanceConfig, EnhancedPerformanceRunner};
use qitops::reporting::{ReportFormat, ReportGenerator};
use qitops::security::{SecurityTestConfig, SecurityTestRunner};
use qitops::web::{WebTestConfig, WebTestRunner};

/// QitOps - Quality Assurance Testing Tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Generate report in specified format
    #[arg(short, long)]
    report: Option<String>,

    /// Output path for the report
    #[arg(short, long)]
    output: Option<String>,

    /// Run in CI mode (reduced output, exit code based on test results)
    #[arg(long)]
    ci_mode: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run API tests
    Api {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,
    },
    /// Run API collection tests
    Collection {
        /// Path to the collection configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,

        /// Format output (human, json)
        #[arg(short, long, default_value = "human")]
        format: String,
    },
    /// Run performance tests
    Performance {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,

        /// Number of concurrent users
        #[arg(short, long, default_value = "10")]
        users: u32,

        /// Test duration in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
    /// Run enhanced performance tests with scenarios and load profiles
    PerformanceEnhanced {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,
    },
    /// Run security tests
    Security {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,

        /// Scan depth (1-5)
        #[arg(short, long, default_value = "3")]
        depth: u8,

        /// Include passive scanning
        #[arg(short, long)]
        passive: bool,
    },
    /// Run web tests
    Web {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,

        /// Run in headless mode
        #[arg(long, default_value = "true")]
        headless: bool,

        /// Directory to save screenshots
        #[arg(short = 'S', long)]
        screenshot_dir: Option<String>,
    },
    /// Generate test configuration using AI
    Generate {
        /// Type of test to generate (api, performance, security, web)
        #[arg(short, long)]
        test_type: String,

        /// Description of the test to generate
        #[arg(short, long)]
        description: String,

        /// Output file path for the generated configuration
        #[arg(short, long)]
        output: PathBuf,

        /// AI model to use (llama, mistral, gptj, phi, custom)
        #[arg(short, long, default_value = "llama")]
        model: String,

        /// Path to model weights (required for custom models)
        #[arg(short, long)]
        model_path: Option<String>,
    },
    /// Run data-driven tests
    DataDriven {
        /// Path to the test configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Path to the data source file (CSV or JSON)
        #[arg(short, long)]
        data: PathBuf,

        /// Data source type (csv, json)
        #[arg(short = 't', long, default_value = "csv")]
        data_type: String,

        /// Environment to run tests in
        #[arg(short, long, default_value = "production")]
        environment: String,

        /// Maximum number of iterations to run
        #[arg(short, long)]
        max_iterations: Option<usize>,

        /// Stop on first failure
        #[arg(short = 'f', long)]
        stop_on_failure: bool,
    },
    /// Analyze test results using AI
    Analyze {
        /// Path to test results file(s)
        #[arg(short, long)]
        results: Vec<PathBuf>,

        /// Output file path for the analysis
        #[arg(short, long)]
        output: PathBuf,

        /// AI model to use (llama, mistral, gptj, phi, custom)
        #[arg(short, long, default_value = "llama")]
        model: String,

        /// Path to model weights (required for custom models)
        #[arg(short, long)]
        model_path: Option<String>,
    },
    /// Suggest improvements based on test results using AI
    Improve {
        /// Path to test results file(s)
        #[arg(short, long)]
        results: Vec<PathBuf>,

        /// Output file path for the suggestions
        #[arg(short, long)]
        output: PathBuf,

        /// AI model to use (llama, mistral, gptj, phi, custom)
        #[arg(short, long, default_value = "llama")]
        model: String,

        /// Path to model weights (required for custom models)
        #[arg(short, long)]
        model_path: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger with custom format and appropriate level based on CI mode
    let cli = Cli::parse();

    let log_level = if cli.ci_mode {
        // In CI mode, only show warnings and errors by default
        "warn"
    } else {
        // In interactive mode, show info and above by default
        "info"
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
        .format_timestamp_millis()
        .init();

    // Log CI mode status
    if cli.ci_mode {
        info!("Running in CI mode");
    }

    // Determine report format if specified
    let report_format = if let Some(format) = &cli.report {
        match format.to_lowercase().as_str() {
            "json" => Some(ReportFormat::Json),
            "xml" => Some(ReportFormat::Xml),
            "html" => Some(ReportFormat::Html),
            "csv" => Some(ReportFormat::Csv),
            _ => {
                eprintln!("Unsupported report format: {}. Using default format.", format);
                None
            }
        }
    } else {
        None
    };

    // Store test results for reporting
    let mut test_results = Vec::new();

    let result = match &cli.command {
        Commands::Api { config, environment: _ } => {
            info!("Running API tests with config: {}", config.display());
            let test_config: ApiTestConfig = load_config(config)?;
            let runner = ApiTestRunner::new();
            let result = runner.run(&test_config).await?;

            // Print results
            if !cli.ci_mode {
                println!("\nTest Results:");
                println!("Name: {}", result.name);
                println!("Status: {}", result.status);
                println!("Duration: {:.2}s", result.duration);
                if let Some(details) = &result.details {
                    println!("Details: {}", serde_json::to_string_pretty(details)?);
                }
                println!("Timestamp: {}", result.timestamp);
            } else {
                // In CI mode, print minimal output
                println!("API Test: {} - {} ({:.2}s)", result.name, result.status, result.duration);
            }

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Performance { config, environment: _, users, duration } => {
            info!("Running performance tests with config: {}", config.display());
            let test_config: PerformanceTestConfig = load_config(config)?;
            let runner = PerformanceTestRunner::new(*users, *duration);
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nPerformance Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = &result.details {
                println!("Details: {}", serde_json::to_string_pretty(details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::PerformanceEnhanced { config, environment: _ } => {
            info!("Running enhanced performance tests with config: {}", config.display());
            let test_config: EnhancedPerformanceConfig = load_config(config)?;
            let runner = EnhancedPerformanceRunner::new();
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nEnhanced Performance Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);

            if let Some(details) = &result.details {
                // Print a summary of the metrics
                if let Some(metrics) = details.get("metrics") {
                    println!("\nMetrics Summary:");

                    // Print total requests and success rate
                    let total_requests = metrics["total_requests"].as_u64().unwrap_or(0);
                    let success_count = metrics["success_count"].as_u64().unwrap_or(0);
                    let error_count = metrics["error_count"].as_u64().unwrap_or(0);

                    println!("  Total Requests: {}", total_requests);
                    println!("  Success Count: {}", success_count);
                    println!("  Error Count: {}", error_count);
                    println!("  Success Rate: {:.2}%",
                        (success_count as f64 / total_requests as f64) * 100.0);

                    // Print response time metrics
                    if let Some(response_time) = metrics.get("response_time") {
                        println!("\nResponse Time:");
                        println!("  Average: {:.2}ms", response_time["avg"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  Min: {:.2}ms", response_time["min"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  Max: {:.2}ms", response_time["max"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  p50: {:.2}ms", response_time["p50"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  p90: {:.2}ms", response_time["p90"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  p95: {:.2}ms", response_time["p95"].as_f64().unwrap_or(0.0) * 1000.0);
                        println!("  p99: {:.2}ms", response_time["p99"].as_f64().unwrap_or(0.0) * 1000.0);
                    }

                    // Print scenario metrics
                    if let Some(scenarios) = metrics.get("scenarios") {
                        println!("\nScenario Results:");
                        for (name, scenario) in scenarios.as_object().unwrap() {
                            println!("  {}: {}/{} requests successful ({:.2}%)",
                                name,
                                scenario["success_count"].as_u64().unwrap_or(0),
                                scenario["total_requests"].as_u64().unwrap_or(0),
                                scenario["success_rate"].as_f64().unwrap_or(0.0));
                        }
                    }

                    // Print threshold results
                    if let Some(thresholds) = details.get("thresholds") {
                        println!("\nThresholds:");
                        for threshold in thresholds.as_array().unwrap() {
                            println!("  {}: {} - {}",
                                threshold["metric"].as_str().unwrap_or(""),
                                threshold["expression"].as_str().unwrap_or(""),
                                if threshold["passed"].as_bool().unwrap_or(false) { "PASSED" } else { "FAILED" });
                        }
                    }
                }

                // Option to print full details
                println!("\nFor full details, use the --report option to generate a JSON report.");
            }

            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Security { config, environment: _, depth, passive } => {
            info!("Running security tests with config: {}", config.display());
            let test_config: SecurityTestConfig = load_config(config)?;
            let runner = SecurityTestRunner::new(*depth, *passive);
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nSecurity Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = &result.details {
                println!("Details: {}", serde_json::to_string_pretty(details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Web { config, environment: _, headless, screenshot_dir } => {
            info!("Running web tests with config: {}", config.display());
            let test_config: WebTestConfig = load_config(config)?;
            let runner = WebTestRunner::new(*headless, screenshot_dir.clone());
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nWeb Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = &result.details {
                println!("Details: {}", serde_json::to_string_pretty(details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Collection { config, environment, format } => {
            info!("Running API collection with config: {}", config.display());
            let runner = ApiCollectionRunner::new();
            let collection = ApiCollectionRunner::load_collection(config)?;
            let result = runner.run_collection(&collection, environment).await?;

            // Print results based on format
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                },
                _ => { // human format
                    println!("\nCollection Results:");
                    println!("Name: {}", result.name);
                    println!("Status: {}", result.status);
                    println!("Duration: {:.2}s", result.duration);
                    println!("Timestamp: {}", result.timestamp);
                    println!("\nRequest Results:");

                    for (i, req_result) in result.request_results.iter().enumerate() {
                        println!("  {}. {} - {}", i + 1, req_result.name, req_result.status);
                    }

                    println!("\nCaptured Variables:");
                    for (key, value) in &result.variables {
                        println!("  {}: {}", key, value);
                    }
                }
            }

            // Store individual test results for reporting
            test_results.extend(result.request_results);

            Ok(())
        }
        Commands::Generate { test_type, description, output, model, model_path } => {
            info!("Generating {} test configuration using AI", test_type);

            // Create AI configuration
            let model_type = match model.to_lowercase().as_str() {
                "llama" => AiModelType::Llama,
                "mistral" => AiModelType::Mistral,
                "gptj" => AiModelType::GptJ,
                "phi" => AiModelType::Phi,
                "custom" => AiModelType::Custom,
                _ => return Err(Error::ValidationError(format!("Unsupported AI model: {}", model))),
            };

            let ai_config = AiConfig {
                model_type,
                model_path: model_path.clone(),
                context_size: 2048,
                temperature: 0.7,
                max_tokens: 1024,
                system_prompt: None,
            };

            let generator = AiTestGenerator::new(ai_config);
            let json_config = generator.generate_test_config(description, test_type).await?;

            // Save the generated configuration
            std::fs::write(output, json_config)?;
            println!("\nTest configuration generated successfully!");
            println!("Saved to: {}", output.display());

            Ok(())
        }
        Commands::Analyze { results, output, model, model_path } => {
            info!("Analyzing test results using AI");

            // Create AI configuration
            let model_type = match model.to_lowercase().as_str() {
                "llama" => AiModelType::Llama,
                "mistral" => AiModelType::Mistral,
                "gptj" => AiModelType::GptJ,
                "phi" => AiModelType::Phi,
                "custom" => AiModelType::Custom,
                _ => return Err(Error::ValidationError(format!("Unsupported AI model: {}", model))),
            };

            let ai_config = AiConfig {
                model_type,
                model_path: model_path.clone(),
                context_size: 2048,
                temperature: 0.7,
                max_tokens: 1024,
                system_prompt: None,
            };

            // Load test results
            let mut loaded_results = Vec::new();
            for result_path in results {
                let content = std::fs::read_to_string(&result_path)?;
                let result: qitops::common::TestResult = serde_json::from_str(&content)?;
                loaded_results.push(result);
            }

            let generator = AiTestGenerator::new(ai_config);
            let analysis = generator.analyze_test_results(&loaded_results).await?;

            // Save the analysis
            std::fs::write(output, analysis)?;
            println!("\nTest results analysis completed successfully!");
            println!("Saved to: {}", output.display());

            Ok(())
        }
        Commands::Improve { results, output, model, model_path } => {
            info!("Generating improvement suggestions using AI");

            // Create AI configuration
            let model_type = match model.to_lowercase().as_str() {
                "llama" => AiModelType::Llama,
                "mistral" => AiModelType::Mistral,
                "gptj" => AiModelType::GptJ,
                "phi" => AiModelType::Phi,
                "custom" => AiModelType::Custom,
                _ => return Err(Error::ValidationError(format!("Unsupported AI model: {}", model))),
            };

            let ai_config = AiConfig {
                model_type,
                model_path: model_path.clone(),
                context_size: 2048,
                temperature: 0.7,
                max_tokens: 1024,
                system_prompt: None,
            };

            // Load test results
            let mut loaded_results = Vec::new();
            for result_path in results {
                let content = std::fs::read_to_string(&result_path)?;
                let result: qitops::common::TestResult = serde_json::from_str(&content)?;
                loaded_results.push(result);
            }

            let generator = AiTestGenerator::new(ai_config);
            let suggestions = generator.suggest_improvements(&loaded_results).await?;

            // Save the suggestions
            std::fs::write(output, suggestions)?;
            println!("\nImprovement suggestions generated successfully!");
            println!("Saved to: {}", output.display());

            Ok(())
        },
        Commands::DataDriven { config, data, data_type, environment, max_iterations, stop_on_failure } => {
            info!("Running data-driven tests with config: {}", config.display());

            // Determine the test type from the config file
            let test_type = qitops::schema::determine_test_type(&config)?;
            info!("Detected test type: {}", test_type);

            // Create data-driven configuration
            let data_source_type = match data_type.to_lowercase().as_str() {
                "csv" => qitops::data_driven::DataSourceType::Csv,
                "json" => qitops::data_driven::DataSourceType::Json,
                _ => return Err(Error::ValidationError(format!("Unsupported data source type: {}", data_type))),
            };

            let data_source = qitops::data_driven::DataSource {
                source_type: data_source_type,
                file_path: Some(data.to_string_lossy().to_string()),
                delimiter: ",".to_string(),
                has_header: true,
                json_path: "$".to_string(),
                data: None,
            };

            let data_driven_config = DataDrivenConfig {
                data_source,
                stop_on_failure: *stop_on_failure,
                max_iterations: *max_iterations,
                placeholders: None,
            };

            // Create data-driven runner
            let runner = DataDrivenRunner::new(data_driven_config)?;
            info!("Loaded {} data rows", runner.row_count());

            let max_iter = runner.max_iterations();
            info!("Running up to {} iterations", max_iter);

            // Run tests for each data row
            let mut success_count = 0;
            let mut failure_count = 0;
            let mut all_results = Vec::new();

            for (i, row) in runner.data_rows().iter().take(max_iter).enumerate() {
                info!("Running iteration {} of {}", i + 1, max_iter);

                // Load the appropriate test configuration based on test type
                let result = match test_type.as_str() {
                    "api" => {
                        let base_config: ApiTestConfig = load_config(&config)?;
                        let test_config = runner.apply_replacements(&base_config, row)?;
                        let api_runner = ApiTestRunner::new();
                        api_runner.run(&test_config).await
                    },
                    "performance" => {
                        let base_config: PerformanceTestConfig = load_config(&config)?;
                        let test_config = runner.apply_replacements(&base_config, row)?;
                        let perf_runner = PerformanceTestRunner::new(10, 30); // Default values
                        perf_runner.run(&test_config).await
                    },
                    "performance_enhanced" => {
                        let base_config: EnhancedPerformanceConfig = load_config(&config)?;
                        let test_config = runner.apply_replacements(&base_config, row)?;
                        let perf_runner = EnhancedPerformanceRunner::new();
                        perf_runner.run(&test_config).await
                    },
                    "security" => {
                        let base_config: SecurityTestConfig = load_config(&config)?;
                        let test_config = runner.apply_replacements(&base_config, row)?;
                        let sec_runner = SecurityTestRunner::new(3, false); // Default values
                        sec_runner.run(&test_config).await
                    },
                    "web" => {
                        let base_config: WebTestConfig = load_config(&config)?;
                        let test_config = runner.apply_replacements(&base_config, row)?;
                        let web_runner = WebTestRunner::new(true, None); // Default values
                        web_runner.run(&test_config).await
                    },
                    "collection" => {
                        let collection = ApiCollectionRunner::load_collection(&config)?;
                        // Apply replacements to the collection
                        let collection_json = serde_json::to_value(&collection)?;
                        let mut collection_json_mut = collection_json.clone();
                        // Replace placeholders in the collection JSON
                        qitops::data_driven::replace_placeholders(&mut collection_json_mut, row);
                        let updated_collection: qitops::api_collection::ApiCollection = serde_json::from_value(collection_json_mut)?;

                        let api_runner = ApiCollectionRunner::new();
                        let collection_result = api_runner.run_collection(&updated_collection, environment).await?;

                        // Convert collection result to test result
                        // Get the first result or create a default one
                        if let Some(result) = collection_result.request_results.first() {
                            Ok(result.clone())
                        } else {
                            Ok(qitops::common::TestResult {
                                name: "No results".to_string(),
                                status: "failed".to_string(),
                                duration: 0.0,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                                details: Some(serde_json::json!({})),
                            })
                        }
                    },
                    _ => return Err(Error::ValidationError(format!("Unsupported test type: {}", test_type))),
                };

                match result {
                    Ok(test_result) => {
                        // Print result
                        if !cli.ci_mode {
                            println!("\nIteration {} Results:", i + 1);
                            println!("Name: {}", test_result.name);
                            println!("Status: {}", test_result.status);
                            println!("Duration: {:.2}s", test_result.duration);
                            if let Some(details) = &test_result.details {
                                println!("Details: {}", serde_json::to_string_pretty(details)?);
                            }
                            println!("Timestamp: {}", test_result.timestamp);

                            // Print data row
                            println!("\nData Row:");
                            for (key, value) in row {
                                println!("  {}: {}", key, value);
                            }
                            println!();
                        } else {
                            // In CI mode, print minimal output
                            println!("Iteration {}: {} - {}", i + 1, test_result.name, test_result.status);
                        }

                        // Update counters
                        if test_result.status == "passed" {
                            success_count += 1;
                        } else {
                            failure_count += 1;
                            if runner.stop_on_failure() {
                                warn!("Stopping on first failure");
                                break;
                            }
                        }

                        // Store result
                        all_results.push(test_result);
                        test_results.push(all_results.last().unwrap().clone());
                    },
                    Err(e) => {
                        eprintln!("Error in iteration {}: {}", i + 1, e);
                        failure_count += 1;
                        if runner.stop_on_failure() {
                            warn!("Stopping on first failure");
                            break;
                        }
                    }
                }
            }

            // Print summary
            println!("\nData-Driven Test Summary:");
            println!("Total Iterations: {}", success_count + failure_count);
            println!("Successful: {}", success_count);
            println!("Failed: {}", failure_count);
            println!("Success Rate: {:.2}%", (success_count as f64 / (success_count + failure_count) as f64) * 100.0);

            Ok(())
        }
    };

    // Generate report if format is specified
    if let Some(format) = report_format {
        if let Some(output_path) = &cli.output {
            info!("Generating report in {:?} format at {}", format, output_path);
            let generator = ReportGenerator::new(format, PathBuf::from(output_path));
            generator.generate(&test_results)?;

            if cli.ci_mode {
                info!("Report generated at {}", output_path);
            } else {
                println!("Report generated successfully at {}", output_path);
            }
        } else {
            eprintln!("Output path must be specified for report generation");
        }
    }

    // In CI mode, exit with non-zero code if any test failed
    if cli.ci_mode {
        let failed_tests = test_results.iter().filter(|r| r.status != "passed").count();

        if failed_tests > 0 {
            eprintln!("{} tests failed", failed_tests);
            std::process::exit(1);
        }
    }

    result
}
