// Models module - Core data structures
// These are placeholder stubs - tests should fail until proper implementation

use serde::{Deserialize, Serialize};

// Placeholder types for contract tests - will be properly implemented later
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub input: String,
    pub context: Option<String>,
    pub shell: ShellType,
    pub safety_level: SafetyLevel,
    pub backend_preference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCommand {
    pub command: String,
    pub explanation: String,
    pub safety_level: RiskLevel,
    pub estimated_impact: Impact,
    pub alternatives: Vec<String>,
    pub backend_used: String,
    pub generation_time_ms: u64,
    pub confidence_score: f32,
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
pub enum SafetyLevel {
    Strict,
    Moderate,
    Permissive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BackendType {
    MLX,
    VLLM,
    Ollama,
    OpenAI,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Impact {
    // Placeholder - will be properly defined later
}

// Types are already public, no re-export needed
