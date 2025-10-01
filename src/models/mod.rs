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

// All types are public through mod.rs exports
