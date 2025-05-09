use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use std::path::PathBuf;

mod ai;
mod api;
mod api_collection;
mod common;
mod error;
mod performance;
mod reporting;
mod schema;
mod security;
mod web;

use ai::{AiConfig, AiModelType, AiTestGenerator};
use api::{ApiTestConfig, ApiTestRunner};
use api_collection::ApiCollectionRunner;
use common::{TestRunner, load_config};
use error::Result;
use performance::{PerformanceTestConfig, PerformanceTestRunner};
use reporting::{ReportFormat, ReportGenerator};
use security::{SecurityTestConfig, SecurityTestRunner};
use web::{WebTestConfig, WebTestRunner};

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
        #[arg(short, long, default_value = "true")]
        headless: bool,

        /// Directory to save screenshots
        #[arg(short, long)]
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
    // Initialize logger with custom format
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    let cli = Cli::parse();

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
        Commands::Api { config, environment } => {
            info!("Running API tests with config: {}", config.display());
            let test_config: ApiTestConfig = load_config(config)?;
            let runner = ApiTestRunner::new();
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nTest Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = result.details {
                println!("Details: {}", serde_json::to_string_pretty(&details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Performance { config, environment, users, duration } => {
            info!("Running performance tests with config: {}", config.display());
            let test_config: PerformanceTestConfig = load_config(config)?;
            let runner = PerformanceTestRunner::new(*users, *duration);
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nPerformance Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = result.details {
                println!("Details: {}", serde_json::to_string_pretty(&details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Security { config, environment, depth, passive } => {
            info!("Running security tests with config: {}", config.display());
            let test_config: SecurityTestConfig = load_config(config)?;
            let runner = SecurityTestRunner::new(*depth, *passive);
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nSecurity Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = result.details {
                println!("Details: {}", serde_json::to_string_pretty(&details)?);
            }
            println!("Timestamp: {}", result.timestamp);

            // Store result for reporting
            test_results.push(result);

            Ok(())
        }
        Commands::Web { config, environment, headless, screenshot_dir } => {
            info!("Running web tests with config: {}", config.display());
            let test_config: WebTestConfig = load_config(config)?;
            let runner = WebTestRunner::new(*headless, screenshot_dir.clone());
            let result = runner.run(&test_config).await?;

            // Print results
            println!("\nWeb Test Results:");
            println!("Name: {}", result.name);
            println!("Status: {}", result.status);
            println!("Duration: {:.2}s", result.duration);
            if let Some(details) = result.details {
                println!("Details: {}", serde_json::to_string_pretty(&details)?);
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
                let result: TestResult = serde_json::from_str(&content)?;
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
                let result: TestResult = serde_json::from_str(&content)?;
                loaded_results.push(result);
            }

            let generator = AiTestGenerator::new(ai_config);
            let suggestions = generator.suggest_improvements(&loaded_results).await?;

            // Save the suggestions
            std::fs::write(output, suggestions)?;
            println!("\nImprovement suggestions generated successfully!");
            println!("Saved to: {}", output.display());

            Ok(())
        }
    };

    // Generate report if format is specified
    if let Some(format) = report_format {
        if let Some(output_path) = &cli.output {
            info!("Generating report in {:?} format at {}", format, output_path);
            let generator = ReportGenerator::new(format, output_path.clone());
            generator.generate(&test_results)?;
            println!("Report generated successfully at {}", output_path);
        } else {
            eprintln!("Output path must be specified for report generation");
        }
    }

    result
}