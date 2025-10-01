// Dangerous command pattern database
// Comprehensive regex patterns for detecting unsafe shell commands

use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::{RiskLevel, ShellType};

use super::DangerPattern;

/// Built-in dangerous patterns loaded once at startup
pub static DANGEROUS_PATTERNS: Lazy<Vec<DangerPattern>> = Lazy::new(|| {
    vec![
        // CRITICAL: Filesystem destruction
        DangerPattern {
            pattern: r"rm\s+(-[rfRF]*\s+)*(/|~|\$HOME|/\*|~/\*)".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Recursive deletion of root or home directory".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"rm\s+-rf\s+/".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Force recursive deletion from root".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"rm\s+-rf\s+--no-preserve-root\s+/".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Bypass root protection and delete everything".to_string(),
            shell_specific: None,
        },
        // CRITICAL: Disk operations
        DangerPattern {
            pattern: r"dd\s+.*if=/dev/(zero|random|urandom).*of=/dev/(sd|hd|nvme)".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Overwrite disk with random data".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"mkfs\.\w+\s+/dev/(sd|hd|nvme)".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Format disk destroying all data".to_string(),
            shell_specific: None,
        },
        // CRITICAL: Fork bombs
        DangerPattern {
            pattern: r":\(\)\s*\{\s*:\s*\|\s*:\s*&\s*\}\s*;\s*:".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Fork bomb - exponential process creation".to_string(),
            shell_specific: Some(ShellType::Bash),
        },
        DangerPattern {
            pattern: r"\|\s*&\s*\|".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Potential fork bomb pattern".to_string(),
            shell_specific: None,
        },
        // HIGH: System directory operations
        DangerPattern {
            pattern: r"(rm|mv|chmod|chown)\s+.*(/bin|/sbin|/usr/bin|/usr/sbin|/etc)".to_string(),
            risk_level: RiskLevel::High,
            description: "Modification of critical system directories".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"chmod\s+777\s+/".to_string(),
            risk_level: RiskLevel::High,
            description: "Recursive permission change from root".to_string(),
            shell_specific: None,
        },
        // HIGH: Privilege escalation
        DangerPattern {
            pattern: r"sudo\s+su\s*$".to_string(),
            risk_level: RiskLevel::High,
            description: "Switch to root user without specific command".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"sudo\s+.*chmod\s+u\+s".to_string(),
            risk_level: RiskLevel::High,
            description: "Adding setuid bit with elevated privileges".to_string(),
            shell_specific: None,
        },
        // HIGH: Download and execute
        DangerPattern {
            pattern: r"(curl|wget)\s+.*\|\s*(bash|sh|zsh|fish)".to_string(),
            risk_level: RiskLevel::High,
            description: "Download and execute remote script without inspection".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"(curl|wget)\s+.*\|\s*sudo\s+(bash|sh)".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Download and execute remote script with root privileges".to_string(),
            shell_specific: None,
        },
        // MODERATE: Package management
        DangerPattern {
            pattern: r"(apt|yum|dnf)\s+remove\s+.*--force".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Force removal of packages bypassing dependencies".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"pip\s+install\s+.*--break-system-packages".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Install Python packages bypassing system protections".to_string(),
            shell_specific: None,
        },
        // MODERATE: Process manipulation
        DangerPattern {
            pattern: r"kill\s+-9\s+(-1|1)\s*$".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Force kill all processes or init".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"killall\s+-9\s+\w+".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Force kill all processes by name".to_string(),
            shell_specific: None,
        },
        // HIGH: Destructive data operations
        DangerPattern {
            pattern: r">\s*/dev/sd[a-z]".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Direct write to disk device".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"shred\s+-[uvz]*\s+/dev/(sd|hd|nvme)".to_string(),
            risk_level: RiskLevel::Critical,
            description: "Securely delete disk device".to_string(),
            shell_specific: None,
        },
        // MODERATE: Network operations
        DangerPattern {
            pattern: r"iptables\s+-F".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Flush all firewall rules".to_string(),
            shell_specific: None,
        },
        DangerPattern {
            pattern: r"ufw\s+disable".to_string(),
            risk_level: RiskLevel::Moderate,
            description: "Disable firewall".to_string(),
            shell_specific: None,
        },
    ]
});

/// Compile all patterns into regex objects at initialization
/// Returns errors for any patterns that fail to compile
pub fn validate_patterns() -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for pattern in DANGEROUS_PATTERNS.iter() {
        if let Err(e) = Regex::new(&pattern.pattern) {
            errors.push(format!("Pattern '{}' failed to compile: {}", pattern.pattern, e));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Get patterns filtered by shell type
pub fn get_patterns_for_shell(shell: ShellType) -> Vec<&'static DangerPattern> {
    DANGEROUS_PATTERNS
        .iter()
        .filter(|p| p.shell_specific.is_none() || p.shell_specific == Some(shell))
        .collect()
}

/// Get patterns filtered by minimum risk level
pub fn get_patterns_by_risk(min_risk: RiskLevel) -> Vec<&'static DangerPattern> {
    DANGEROUS_PATTERNS
        .iter()
        .filter(|p| p.risk_level >= min_risk)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patterns_compile() {
        assert!(validate_patterns().is_ok(), "All patterns should compile");
    }

    #[test]
    fn test_pattern_count() {
        assert!(
            DANGEROUS_PATTERNS.len() >= 20,
            "Should have at least 20 dangerous patterns"
        );
    }

    #[test]
    fn test_shell_specific_filtering() {
        let bash_patterns = get_patterns_for_shell(ShellType::Bash);
        let all_patterns = DANGEROUS_PATTERNS.len();
        assert!(bash_patterns.len() <= all_patterns);
    }

    #[test]
    fn test_risk_filtering() {
        let critical = get_patterns_by_risk(RiskLevel::Critical);
        let high = get_patterns_by_risk(RiskLevel::High);
        let moderate = get_patterns_by_risk(RiskLevel::Moderate);

        assert!(critical.len() <= high.len());
        assert!(high.len() <= moderate.len());
    }

    #[test]
    fn test_critical_patterns_exist() {
        let critical = get_patterns_by_risk(RiskLevel::Critical);
        assert!(!critical.is_empty(), "Should have critical risk patterns");
    }
}
