// Safety Validator Contract
// This defines the interface for command safety validation

use serde::{Deserialize, Serialize};

/// Trait for command safety validation implementations
pub trait SafetyValidator: Send + Sync {
    /// Validate a generated command for safety concerns
    fn validate_command(&self, command: &str, context: &ValidationContext) -> ValidationResult;
    
    /// Get the risk level for a command without full validation details
    fn assess_risk_level(&self, command: &str) -> RiskLevel;
    
    /// Check if a command matches any dangerous patterns
    fn check_dangerous_patterns(&self, command: &str) -> Vec<SafetyViolation>;
    
    /// Validate command syntax for the target shell
    fn validate_syntax(&self, command: &str, shell: ShellType) -> SyntaxValidation;
    
    /// Get user-friendly explanation of why a command is dangerous
    fn explain_risks(&self, command: &str) -> Vec<String>;
}

/// Context information for command validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    pub current_directory: String,
    pub shell_type: ShellType,
    pub user_safety_level: SafetyLevel,
    pub system_info: SystemInfo,
}

/// Result of safety validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_safe: bool,
    pub risk_level: RiskLevel,
    pub violations: Vec<SafetyViolation>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub requires_confirmation: bool,
    pub can_override: bool,
}

/// Specific safety rule violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyViolation {
    pub rule_id: String,
    pub category: SafetyCategory,
    pub severity: RiskLevel,
    pub message: String,
    pub matched_pattern: String,
    pub suggestion: Option<String>,
}

/// Shell syntax validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxValidation {
    pub is_valid: bool,
    pub syntax_errors: Vec<String>,
    pub shell_compatibility: Vec<ShellType>,
    pub posix_compliant: bool,
}

/// System information for context-aware validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: Platform,
    pub has_sudo: bool,
    pub writable_paths: Vec<String>,
    pub protected_paths: Vec<String>,
}

/// Platform types for platform-specific validation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
    FreeBSD,
    Unknown,
}

/// Contract tests for safety validator implementations
pub trait SafetyValidatorContractTests {
    /// Test detection of known dangerous commands
    fn test_dangerous_command_detection(&self);
    
    /// Test risk level assessment accuracy
    fn test_risk_level_assessment(&self);
    
    /// Test POSIX compliance checking
    fn test_posix_compliance(&self);
    
    /// Test shell-specific syntax validation
    fn test_shell_syntax_validation(&self);
    
    /// Test false positive rates (safe commands incorrectly flagged)
    fn test_false_positive_prevention(&self);
    
    /// Test context-aware validation (directory-specific rules)
    fn test_context_aware_validation(&self);
}