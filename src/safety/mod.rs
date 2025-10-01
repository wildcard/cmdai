// Safety module - Command safety validation and risk assessment
// These are placeholder stubs - tests should fail until proper implementation

mod patterns;

use serde::{Deserialize, Serialize};

use crate::models::{RiskLevel, SafetyLevel, ShellType};

pub use patterns::{get_patterns_by_risk, get_patterns_for_shell, validate_patterns};

/// Main safety validator for analyzing command safety
#[derive(Debug)]
pub struct SafetyValidator {
    config: SafetyConfig,
    patterns: Vec<DangerPattern>,
}

/// Configuration for safety validation behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub safety_level: SafetyLevel,
    pub max_command_length: usize,
    pub custom_patterns: Vec<DangerPattern>,
    pub allowlist_patterns: Vec<String>,
}

/// Result of safety validation for a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub allowed: bool,
    pub risk_level: RiskLevel,
    pub explanation: String,
    pub warnings: Vec<String>,
    pub matched_patterns: Vec<String>,
    pub confidence_score: f32,
}

/// Pattern definition for dangerous command detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerPattern {
    pub pattern: String,
    pub risk_level: RiskLevel,
    pub description: String,
    pub shell_specific: Option<ShellType>,
}

impl SafetyValidator {
    /// Create new validator with given configuration
    pub fn new(config: SafetyConfig) -> Result<Self, ValidationError> {
        // Validate configuration
        if config.max_command_length == 0 {
            return Err(ValidationError::InvalidConfig {
                message: "max_command_length must be positive".to_string(),
            });
        }

        // Validate custom patterns can compile
        for pattern in &config.custom_patterns {
            if let Err(e) = regex::Regex::new(&pattern.pattern) {
                return Err(ValidationError::PatternError {
                    pattern: format!("{}: {}", pattern.pattern, e),
                });
            }
        }

        // Load built-in patterns (will be implemented in T012)
        let patterns = config.custom_patterns.clone();

        Ok(Self { config, patterns })
    }

    /// Validate a single command for safety
    pub async fn validate_command(
        &self,
        _command: &str,
        _shell: ShellType,
    ) -> Result<ValidationResult, ValidationError> {
        // Placeholder - will be implemented later
        Err(ValidationError::NotImplemented)
    }

    /// Validate multiple commands efficiently
    pub async fn validate_batch(
        &self,
        _commands: &[String],
        _shell: ShellType,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        // Placeholder - will be implemented later
        Err(ValidationError::NotImplemented)
    }
}

impl SafetyConfig {
    /// Create strict safety configuration (blocks High and Critical)
    pub fn strict() -> Self {
        Self {
            safety_level: SafetyLevel::Strict,
            max_command_length: 1000,
            custom_patterns: Vec::new(),
            allowlist_patterns: Vec::new(),
        }
    }

    /// Create moderate safety configuration (blocks Critical only)
    pub fn moderate() -> Self {
        Self {
            safety_level: SafetyLevel::Moderate,
            max_command_length: 5000,
            custom_patterns: Vec::new(),
            allowlist_patterns: Vec::new(),
        }
    }

    /// Create permissive safety configuration (warns but allows all)
    pub fn permissive() -> Self {
        Self {
            safety_level: SafetyLevel::Permissive,
            max_command_length: 10000,
            custom_patterns: Vec::new(),
            allowlist_patterns: Vec::new(),
        }
    }

    /// Add custom dangerous pattern with validation
    pub fn add_custom_pattern(&mut self, pattern: DangerPattern) -> Result<(), ValidationError> {
        // Validate regex compiles
        regex::Regex::new(&pattern.pattern).map_err(|e| ValidationError::PatternError {
            pattern: format!("{}: {}", pattern.pattern, e),
        })?;

        self.custom_patterns.push(pattern);
        Ok(())
    }

    /// Add allowlist pattern
    pub fn add_allowlist_pattern(&mut self, pattern: impl Into<String>) {
        self.allowlist_patterns.push(pattern.into());
    }
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            safety_level: SafetyLevel::Moderate,
            max_command_length: 1000,
            custom_patterns: Vec::new(),
            allowlist_patterns: Vec::new(),
        }
    }
}

/// Errors that can occur during safety validation
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("Safety validation not implemented yet")]
    NotImplemented,

    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },

    #[error("Pattern compilation failed: {pattern}")]
    PatternError { pattern: String },

    #[error("Validation timeout")]
    Timeout,

    #[error("Internal validation error: {message}")]
    Internal { message: String },
}

// Types are already public, no re-export needed
