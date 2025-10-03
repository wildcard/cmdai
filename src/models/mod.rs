// Models module - Core data structures
// These are placeholder stubs - tests should fail until proper implementation

use serde::{Deserialize, Serialize};

/// Request for command generation from natural language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    /// Natural language description of desired command
    pub input: String,

    /// Target shell type for command generation
    pub shell: ShellType,

    /// User's safety preference level
    pub safety_level: SafetyLevel,

    /// Optional additional context (current directory, environment info)
    pub context: Option<String>,

    /// Optional backend preference
    pub backend_preference: Option<String>,
}

impl CommandRequest {
    /// Create a new command request with the given input and shell type
    pub fn new(input: impl Into<String>, shell: ShellType) -> Self {
        let input = input.into();
        let trimmed = input.trim().to_string();

        Self {
            input: trimmed,
            shell,
            safety_level: SafetyLevel::default(),
            context: None,
            backend_preference: None,
        }
    }

    /// Set the safety level (builder pattern)
    pub fn with_safety(mut self, level: SafetyLevel) -> Self {
        self.safety_level = level;
        self
    }

    /// Set the context (builder pattern)
    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Set the backend preference (builder pattern)
    pub fn with_backend(mut self, backend: impl Into<String>) -> Self {
        self.backend_preference = Some(backend.into());
        self
    }

    /// Validate that the request is well-formed
    pub fn validate(&self) -> Result<(), String> {
        if self.input.is_empty() {
            return Err("Input cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Response from command generation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCommand {
    /// The generated shell command
    pub command: String,

    /// Human-readable explanation of what the command does
    pub explanation: String,

    /// Assessed risk level of the command
    pub safety_level: RiskLevel,

    /// Description of estimated impact
    pub estimated_impact: String,

    /// Alternative commands that could achieve similar results
    pub alternatives: Vec<String>,

    /// Name of the backend that generated this command
    pub backend_used: String,

    /// Time taken to generate the command in milliseconds
    pub generation_time_ms: u64,

    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
}

impl GeneratedCommand {
    /// Validate that the generated command is well-formed
    pub fn validate(&self) -> Result<(), String> {
        if self.command.is_empty() {
            return Err("Command cannot be empty".to_string());
        }
        if self.explanation.is_empty() {
            return Err("Explanation cannot be empty".to_string());
        }
        if !(0.0..=1.0).contains(&self.confidence_score) {
            return Err(format!(
                "Confidence score must be between 0.0 and 1.0, got {}",
                self.confidence_score
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for GeneratedCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;

        writeln!(f, "{}", "Generated Command:".bold())?;
        writeln!(f, "  {}", self.command.bright_cyan().bold())?;
        writeln!(f)?;
        writeln!(f, "{}", "Explanation:".bold())?;
        writeln!(f, "  {}", self.explanation)?;
        writeln!(f)?;
        writeln!(f, "{} {}", "Risk Level:".bold(), self.safety_level)?;
        writeln!(f, "{} {}", "Backend:".bold(), self.backend_used)?;
        writeln!(
            f,
            "{} {:.0}%",
            "Confidence:".bold(),
            self.confidence_score * 100.0
        )?;

        if !self.alternatives.is_empty() {
            writeln!(f)?;
            writeln!(f, "{}", "Alternatives:".bold())?;
            for alt in &self.alternatives {
                writeln!(f, "  • {}", alt.dimmed())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Moderate,
    High,
    Critical,
}

impl RiskLevel {
    /// Check if this risk level requires user confirmation at the given safety level
    pub fn requires_confirmation(&self, safety_level: SafetyLevel) -> bool {
        match safety_level {
            SafetyLevel::Strict => matches!(self, Self::Moderate | Self::High | Self::Critical),
            SafetyLevel::Moderate => matches!(self, Self::High | Self::Critical),
            SafetyLevel::Permissive => matches!(self, Self::Critical),
        }
    }

    /// Check if this risk level should be blocked at the given safety level
    pub fn is_blocked(&self, safety_level: SafetyLevel) -> bool {
        match safety_level {
            SafetyLevel::Strict => matches!(self, Self::High | Self::Critical),
            SafetyLevel::Moderate => matches!(self, Self::Critical),
            SafetyLevel::Permissive => false,
        }
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        match self {
            Self::Safe => write!(f, "{}", "Safe".green()),
            Self::Moderate => write!(f, "{}", "Moderate".yellow()),
            Self::High => write!(f, "{}", "High".bright_red()),
            Self::Critical => write!(f, "{}", "Critical".red().bold()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SafetyLevel {
    /// Blocks High and Critical commands, confirms Moderate
    Strict,
    /// Blocks Critical commands, confirms High
    Moderate,
    /// Warns about all dangerous commands but allows with confirmation
    Permissive,
}

impl Default for SafetyLevel {
    fn default() -> Self {
        Self::Moderate
    }
}

impl std::str::FromStr for SafetyLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strict" => Ok(Self::Strict),
            "moderate" => Ok(Self::Moderate),
            "permissive" => Ok(Self::Permissive),
            _ => Err(format!("Invalid safety level: {}", s)),
        }
    }
}

impl std::fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Moderate => write!(f, "moderate"),
            Self::Permissive => write!(f, "permissive"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BackendType {
    /// Mock backend for testing
    Mock,
    /// Ollama local LLM backend
    Ollama,
    /// vLLM HTTP API backend
    VLlm,
    /// Apple Silicon MLX backend
    Mlx,
}

impl std::str::FromStr for BackendType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mock" => Ok(Self::Mock),
            "ollama" => Ok(Self::Ollama),
            "vllm" => Ok(Self::VLlm),
            "mlx" => Ok(Self::Mlx),
            _ => Err(format!("Unknown backend type: {}", s)),
        }
    }
}

impl std::fmt::Display for BackendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mock => write!(f, "mock"),
            Self::Ollama => write!(f, "ollama"),
            Self::VLlm => write!(f, "vllm"),
            Self::Mlx => write!(f, "mlx"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Sh,
    PowerShell,
    Cmd,
    Unknown,
}

impl ShellType {
    /// Detect the current shell from environment
    pub fn detect() -> Self {
        // Check SHELL environment variable on Unix-like systems
        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("bash") {
                return Self::Bash;
            } else if shell.contains("zsh") {
                return Self::Zsh;
            } else if shell.contains("fish") {
                return Self::Fish;
            } else if shell.ends_with("/sh") {
                return Self::Sh;
            }
        }

        // Check for Windows shells
        #[cfg(target_os = "windows")]
        {
            if std::env::var("PSModulePath").is_ok() {
                return Self::PowerShell;
            }
            return Self::Cmd;
        }

        Self::Unknown
    }

    /// Check if this is a POSIX-compatible shell
    pub fn is_posix(&self) -> bool {
        matches!(self, Self::Bash | Self::Zsh | Self::Fish | Self::Sh)
    }

    /// Check if this is a Windows shell
    pub fn is_windows(&self) -> bool {
        matches!(self, Self::PowerShell | Self::Cmd)
    }
}

impl Default for ShellType {
    fn default() -> Self {
        Self::detect()
    }
}

impl std::str::FromStr for ShellType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            "sh" => Ok(Self::Sh),
            "powershell" | "pwsh" => Ok(Self::PowerShell),
            "cmd" => Ok(Self::Cmd),
            _ => Ok(Self::Unknown),
        }
    }
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Zsh => write!(f, "zsh"),
            Self::Fish => write!(f, "fish"),
            Self::Sh => write!(f, "sh"),
            Self::PowerShell => write!(f, "powershell"),
            Self::Cmd => write!(f, "cmd"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Backend metadata for diagnostics and selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendInfo {
    /// Type of backend
    pub backend_type: BackendType,

    /// Name of the model being used
    pub model_name: String,

    /// Whether this backend supports streaming responses
    pub supports_streaming: bool,

    /// Maximum number of tokens the model can generate
    pub max_tokens: u32,

    /// Typical latency in milliseconds
    pub typical_latency_ms: u64,

    /// Memory usage in megabytes
    pub memory_usage_mb: u64,

    /// Backend version string
    pub version: String,
}

impl BackendInfo {
    /// Validate that backend info has reasonable values
    pub fn validate(&self) -> Result<(), String> {
        if self.model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        if self.max_tokens == 0 {
            return Err("Max tokens must be positive".to_string());
        }
        if self.version.is_empty() {
            return Err("Version cannot be empty".to_string());
        }
        Ok(())
    }
}

// All types are public through mod.rs exports

// ============================================================================
// Infrastructure Models (Feature 003)
// ============================================================================

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::PathBuf;

/// Platform operating system type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
}

impl Platform {
    /// Detect current platform at runtime
    pub fn detect() -> Self {
        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        compile_error!("Unsupported platform");
    }

    /// Check if platform is POSIX-compliant
    pub fn is_posix(&self) -> bool {
        matches!(self, Platform::Linux | Platform::MacOS)
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOS => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
        }
    }
}

/// Log severity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Convert to tracing Level
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }

    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" | "err" => Ok(LogLevel::Error),
            _ => Err(format!(
                "Invalid log level '{}'. Valid options: debug, info, warn, error",
                s
            )),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
