// CLI module - Command-line interface and user interaction
// These are placeholder stubs - tests should fail until proper implementation

use serde::{Deserialize, Serialize};

use crate::models::{SafetyLevel, ShellType};

/// Main CLI application struct
#[derive(Debug)]
pub struct CliApp {
    #[allow(dead_code)] // Will be used in Module D (CLI Interface) implementation
    config: CliConfig,
}

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub default_shell: ShellType,
    pub safety_level: SafetyLevel,
    pub output_format: OutputFormat,
    pub auto_confirm: bool,
}

/// Result of CLI command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliResult {
    pub generated_command: String,
    pub explanation: String,
    pub executed: bool,
    pub blocked_reason: Option<String>,
    pub requires_confirmation: bool,
    pub confirmation_prompt: String,
    pub alternatives: Vec<String>,
    pub shell_used: ShellType,
    pub output_format: OutputFormat,
    pub debug_info: Option<String>,
    pub generation_details: String,
    pub timing_info: TimingInfo,
    pub warnings: Vec<String>,
    pub detected_context: String,
}

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Yaml,
    Plain,
}

/// Timing information for performance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingInfo {
    pub generation_time_ms: u64,
    pub execution_time_ms: u64,
    pub total_time_ms: u64,
}

impl CliApp {
    /// Create new CLI application instance
    pub async fn new() -> Result<Self, CliError> {
        // Placeholder - will be implemented later
        Err(CliError::NotImplemented)
    }

    /// Run CLI with provided arguments
    pub async fn run_with_args<T>(&self, _args: T) -> Result<CliResult, CliError> {
        // Placeholder - will be implemented later
        Err(CliError::NotImplemented)
    }

    /// Show help information
    pub async fn show_help(&self) -> Result<String, CliError> {
        // Placeholder - will be implemented later
        Err(CliError::NotImplemented)
    }

    /// Show version information
    pub async fn show_version(&self) -> Result<String, CliError> {
        // Placeholder - will be implemented later
        Err(CliError::NotImplemented)
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            default_shell: ShellType::Bash,
            safety_level: SafetyLevel::Moderate,
            output_format: OutputFormat::Plain,
            auto_confirm: false,
        }
    }
}

impl Default for TimingInfo {
    fn default() -> Self {
        Self {
            generation_time_ms: 0,
            execution_time_ms: 0,
            total_time_ms: 0,
        }
    }
}

/// Errors that can occur during CLI operations
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum CliError {
    #[error("CLI functionality not implemented yet")]
    NotImplemented,

    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Command generation failed: {details}")]
    GenerationFailed { details: String },

    #[error("Command execution failed: {details}")]
    ExecutionFailed { details: String },

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Internal CLI error: {message}")]
    Internal { message: String },
}

// Types are already public, no re-export needed
