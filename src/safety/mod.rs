// Safety module - Command safety validation and risk assessment
// These are placeholder stubs - tests should fail until proper implementation

mod patterns;

use serde::{Deserialize, Serialize};

use crate::models::{RiskLevel, SafetyLevel, ShellType};

pub use patterns::{
    get_compiled_patterns_for_shell, get_patterns_by_risk, get_patterns_for_shell,
    validate_patterns,
};

/// Main safety validator for analyzing command safety
#[derive(Debug)]
pub struct SafetyValidator {
    config: SafetyConfig,
    patterns: Vec<DangerPattern>,
    // Cached compiled regex patterns for performance
    compiled_patterns: Vec<(regex::Regex, RiskLevel, String)>,
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

        // Pre-compile all custom patterns for performance
        let mut compiled_patterns = Vec::new();
        for pattern in &config.custom_patterns {
            match regex::Regex::new(&pattern.pattern) {
                Ok(regex) => {
                    compiled_patterns.push((
                        regex,
                        pattern.risk_level,
                        pattern.description.clone(),
                    ));
                }
                Err(e) => {
                    return Err(ValidationError::PatternError {
                        pattern: format!("{}: {}", pattern.pattern, e),
                    });
                }
            }
        }

        let patterns = config.custom_patterns.clone();

        Ok(Self {
            config,
            patterns,
            compiled_patterns,
        })
    }

    /// Check if command contains dangerous pattern in executable context
    /// (not just in a quoted string)
    fn is_dangerous_in_context(command: &str, pattern_regex: &regex::Regex) -> bool {
        // Simple heuristic: if the pattern match is inside quotes, it's likely safe
        // This prevents false positives like: echo 'rm -rf /' > script.sh

        if !pattern_regex.is_match(command) {
            return false;
        }

        // Find all matches
        for mat in pattern_regex.find_iter(command) {
            let match_start = mat.start();
            let before = &command[..match_start];

            // Count unescaped quotes before the match
            let single_quotes = before.matches('\'').count() - before.matches("\\'").count();
            let double_quotes = before.matches('"').count() - before.matches("\\\"").count();

            // If odd number of quotes, we're inside a string literal
            if single_quotes % 2 == 1 || double_quotes % 2 == 1 {
                continue;
            }

            // Match is in executable context
            return true;
        }

        false
    }

    /// Validate a single command for safety
    pub async fn validate_command(
        &self,
        command: &str,
        shell: ShellType,
    ) -> Result<ValidationResult, ValidationError> {
        // Check command length
        if command.len() > self.config.max_command_length {
            return Ok(ValidationResult {
                allowed: false,
                risk_level: RiskLevel::Moderate,
                explanation: format!(
                    "Command exceeds maximum length of {} characters",
                    self.config.max_command_length
                ),
                warnings: vec![format!(
                    "Command is {} characters long (max: {})",
                    command.len(),
                    self.config.max_command_length
                )],
                matched_patterns: vec![],
                confidence_score: 1.0,
            });
        }

        // Check allowlist patterns first
        for allow_pattern in &self.config.allowlist_patterns {
            if let Ok(regex) = regex::Regex::new(allow_pattern) {
                if regex.is_match(command) {
                    return Ok(ValidationResult {
                        allowed: true,
                        risk_level: RiskLevel::Safe,
                        explanation: "Command matches allowlist pattern".to_string(),
                        warnings: vec![],
                        matched_patterns: vec![allow_pattern.clone()],
                        confidence_score: 1.0,
                    });
                }
            }
        }

        // Get pre-compiled patterns for this shell type
        let built_in_patterns = patterns::get_compiled_patterns_for_shell(shell);
        let mut matched = Vec::new();
        let mut highest_risk = RiskLevel::Safe;
        let mut warnings = Vec::new();

        // Check against built-in compiled patterns (fast!)
        for (regex, risk_level, description, _) in built_in_patterns {
            if Self::is_dangerous_in_context(command, regex) {
                matched.push(description.clone());
                if *risk_level > highest_risk {
                    highest_risk = *risk_level;
                }
                warnings.push(format!("{}: {}", risk_level, description));
            }
        }

        // Check pre-compiled custom patterns
        for (regex, risk_level, description) in &self.compiled_patterns {
            if Self::is_dangerous_in_context(command, regex) {
                matched.push(description.clone());
                if *risk_level > highest_risk {
                    highest_risk = *risk_level;
                }
                warnings.push(format!("{}: {}", risk_level, description));
            }
        }

        // Determine if command is allowed based on safety level
        // Commands are not allowed if either blocked OR require confirmation
        let requires_confirm = highest_risk.requires_confirmation(self.config.safety_level);
        let is_blocked = highest_risk.is_blocked(self.config.safety_level);
        let allowed = !is_blocked && !requires_confirm;

        // Generate explanation
        let explanation = if matched.is_empty() {
            "No dangerous patterns detected".to_string()
        } else {
            // Include specific risk types in explanation
            let risk_keywords: Vec<&str> = matched
                .iter()
                .flat_map(|desc| {
                    let lower = desc.to_lowercase();
                    let mut keywords = Vec::new();
                    if lower.contains("delet") {
                        keywords.push("deletion");
                    }
                    if lower.contains("remov") {
                        keywords.push("removal");
                    }
                    if lower.contains("recursive") {
                        keywords.push("recursive");
                    }
                    if lower.contains("privilege") || lower.contains("root") || lower.contains("sudo")
                    {
                        keywords.push("privilege escalation");
                    }
                    if lower.contains("network") || lower.contains("backdoor") {
                        keywords.push("network");
                    }
                    if lower.contains("disk") || lower.contains("format") {
                        keywords.push("disk");
                    }
                    keywords
                })
                .collect();

            let risk_types = if risk_keywords.is_empty() {
                String::new()
            } else {
                let unique: std::collections::HashSet<_> = risk_keywords.into_iter().collect();
                format!(" ({})", unique.into_iter().collect::<Vec<_>>().join(", "))
            };

            format!(
                "Detected {} dangerous pattern(s) at {} risk level{}",
                matched.len(),
                highest_risk,
                risk_types
            )
        };

        // Calculate confidence score based on pattern matches
        let confidence_score = if matched.is_empty() {
            0.95 // High confidence for safe commands
        } else {
            1.0 // Very confident about dangerous patterns
        };

        Ok(ValidationResult {
            allowed,
            risk_level: highest_risk,
            explanation,
            warnings,
            matched_patterns: matched,
            confidence_score,
        })
    }

    /// Validate multiple commands efficiently
    pub async fn validate_batch(
        &self,
        commands: &[String],
        shell: ShellType,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let mut results = Vec::with_capacity(commands.len());

        for command in commands {
            let result = self.validate_command(command, shell).await?;
            results.push(result);
        }

        Ok(results)
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

    /// Add custom dangerous pattern (validation happens in SafetyValidator::new)
    pub fn add_custom_pattern(&mut self, pattern: DangerPattern) -> Result<(), ValidationError> {
        // Note: We defer regex validation to SafetyValidator::new() to allow
        // configuration building, but we do a quick check here for immediate feedback
        if let Err(e) = regex::Regex::new(&pattern.pattern) {
            // Return error but still add for deferred validation
            self.custom_patterns.push(pattern);
            return Err(ValidationError::PatternError {
                pattern: format!("{}: {}", &self.custom_patterns.last().unwrap().pattern, e),
            });
        }

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
