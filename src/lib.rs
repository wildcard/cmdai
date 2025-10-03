//! cmdai - Natural Language to Shell Command CLI Tool
//!
//! This library provides core functionality for converting natural language
//! descriptions into safe, POSIX-compliant shell commands using local LLMs.
//!
//! # Core Modules
//!
//! - [`models`] - Core data types (CommandRequest, GeneratedCommand, enums)
//! - [`safety`] - Safety validation with dangerous command detection
//! - [`backends`] - Command generation backends (Mock, Ollama, vLLM, MLX)
//! - [`cli`] - CLI interface and argument parsing
//! - [`cache`] - Model caching with integrity validation
//! - [`config`] - Configuration management with TOML support
//! - [`execution`] - Execution context capture and shell detection
//! - [`logging`] - Structured logging with sensitive data redaction
//!
//! # Example
//!
//! ```no_run
//! use cmdai::models::{CommandRequest, ShellType, SafetyLevel};
//!
//! let request = CommandRequest::new("list all files", ShellType::Bash)
//!     .with_safety(SafetyLevel::Moderate);
//! ```

pub mod backends;
pub mod cache;
pub mod cli;
pub mod config;
pub mod execution;
pub mod logging;
pub mod models;
pub mod safety;

// Re-export commonly used types for convenience
pub use models::{
    BackendInfo, BackendType, CacheManifest, CachedModel, CommandRequest, ConfigSchema,
    ExecutionContext, GeneratedCommand, LogEntry, LogLevel, Platform, RiskLevel, SafetyLevel,
    ShellType, UserConfiguration, UserConfigurationBuilder,
};

// Re-export infrastructure module types and errors
pub use cache::{CacheError, CacheManager, CacheStats, IntegrityReport};
pub use config::{ConfigError, ConfigManager};
pub use execution::{ExecutionError, PlatformDetector, ShellDetector};
pub use logging::{LogConfig, LogConfigBuilder, LogError, LogFormat, LogOutput, Logger, Redaction};
