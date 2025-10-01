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
pub mod core;
pub mod execution;
pub mod logging;
pub mod models;
pub mod perf;
pub mod platform;
pub mod safety;

// Re-export commonly used types for convenience
pub use models::{
    BackendInfo, BackendType, CommandRequest, GeneratedCommand, RiskLevel, SafetyLevel, ShellType,
};
