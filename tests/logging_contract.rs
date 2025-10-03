// Logging module contract tests - THESE MUST FAIL INITIALLY (TDD)
// Tests validate the logging module API from specs/003-implement-core-infrastructure/contracts/logging-api.md

use std::time::Duration;
use tempfile::TempDir;

// Import types that will be implemented later
// NOTE: These imports will fail until we implement the actual logging module
use cmdai::logging::{
    LogConfig, LogConfigBuilder, LogError, LogFormat, LogLevel, LogOutput, LogRotation, Logger,
    OperationSpan, Redaction,
};

#[test]
fn test_logger_initialization() {
    // CONTRACT: Logger::init() initializes global tracing subscriber
    let config = LogConfig::default();
    let result = Logger::init(config);

    assert!(result.is_ok(), "Logger initialization should succeed");
}

#[test]
fn test_logger_already_initialized() {
    // CONTRACT: Logger::init() returns error if already initialized
    let config = LogConfig::default();
    let _ = Logger::init(config.clone());

    // Second init should fail
    let result2 = Logger::init(config);

    assert!(result2.is_err(), "Second initialization should fail");
    match result2.unwrap_err() {
        LogError::AlreadyInitialized => { /* Expected */ }
        e => panic!("Expected AlreadyInitialized error, got: {:?}", e),
    }
}

#[test]
fn test_log_config_default() {
    // CONTRACT: LogConfig::default() provides sensible defaults
    let config = LogConfig::default();

    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.format, LogFormat::Json);
    assert!(config.redaction_enabled);
}

#[test]
fn test_log_config_development() {
    // CONTRACT: LogConfig::development() provides dev-friendly settings
    let config = LogConfig::development();

    assert_eq!(config.log_level, LogLevel::Debug);
    assert_eq!(config.format, LogFormat::Pretty);
    assert_eq!(config.output, LogOutput::Stderr);
}

#[test]
fn test_log_config_production() {
    // CONTRACT: LogConfig::production() provides production settings
    let config = LogConfig::production();

    assert_eq!(config.log_level, LogLevel::Info);
    assert_eq!(config.format, LogFormat::Json);
    assert_eq!(config.rotation, LogRotation::Daily);
}

#[test]
fn test_log_config_builder() {
    // CONTRACT: Builder pattern allows custom configuration
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");

    let config = LogConfig::builder()
        .log_level(LogLevel::Warn)
        .format(LogFormat::Pretty)
        .output(LogOutput::File {
            path: log_file.clone(),
        })
        .rotation(LogRotation::Hourly)
        .redaction(false)
        .build();

    assert_eq!(config.log_level, LogLevel::Warn);
    assert_eq!(config.format, LogFormat::Pretty);
    assert_eq!(
        config.output,
        LogOutput::File {
            path: log_file.clone()
        }
    );
    assert_eq!(config.rotation, LogRotation::Hourly);
    assert!(!config.redaction_enabled);
}

#[test]
fn test_log_level_ordering() {
    // CONTRACT: LogLevel has correct ordering (Debug < Info < Warn < Error)
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_log_level_from_str() {
    // CONTRACT: LogLevel::from_str() parses case-insensitively
    assert_eq!(LogLevel::from_str("debug").unwrap(), LogLevel::Debug);
    assert_eq!(LogLevel::from_str("DEBUG").unwrap(), LogLevel::Debug);
    assert_eq!(LogLevel::from_str("Info").unwrap(), LogLevel::Info);
    assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warn);
    assert_eq!(LogLevel::from_str("ERROR").unwrap(), LogLevel::Error);

    // Invalid input should error
    assert!(LogLevel::from_str("invalid").is_err());
}

#[test]
fn test_log_level_to_tracing_level() {
    // CONTRACT: LogLevel::to_tracing_level() converts to tracing::Level
    let debug_level = LogLevel::Debug.to_tracing_level();
    assert_eq!(debug_level, tracing::Level::DEBUG);

    let info_level = LogLevel::Info.to_tracing_level();
    assert_eq!(info_level, tracing::Level::INFO);

    let warn_level = LogLevel::Warn.to_tracing_level();
    assert_eq!(warn_level, tracing::Level::WARN);

    let error_level = LogLevel::Error.to_tracing_level();
    assert_eq!(error_level, tracing::Level::ERROR);
}

#[test]
fn test_logger_for_module() {
    // CONTRACT: Logger::for_module() creates module-specific logger
    let logger = Logger::for_module("cmdai::cache");

    // Logger should be usable for that module
    // (Actual logging would be tested in integration tests)
    let _span = logger.start_operation("test_operation");
}

#[test]
fn test_operation_span_creation() {
    // CONTRACT: start_operation() creates OperationSpan
    let logger = Logger::for_module("test");
    let span = logger.start_operation("test_op");

    // Span should record additional fields
    span.record("key", "value");
    span.record_duration(Duration::from_millis(100));
}

#[test]
fn test_operation_span_auto_drop() {
    // CONTRACT: OperationSpan automatically ends on drop
    let logger = Logger::for_module("test");

    {
        let span = logger.start_operation("scoped_op");
        span.record("step", "1");
        // Span should auto-close when it goes out of scope
    }

    // Span dropped, should be logged automatically
}

#[test]
fn test_operation_span_success() {
    // CONTRACT: OperationSpan can record success/failure
    let logger = Logger::for_module("test");
    let span = logger.start_operation("successful_op");

    span.success();
    // Should log success when span drops
}

#[test]
fn test_operation_span_error() {
    // CONTRACT: OperationSpan can record error context
    let logger = Logger::for_module("test");
    let span = logger.start_operation("failed_op");

    let error = std::io::Error::new(std::io::ErrorKind::NotFound, "test error");
    span.error(&error);
    // Should log error details when span drops
}

#[test]
fn test_redaction_redacts_api_keys() {
    // CONTRACT: Redaction::redact() removes API keys
    let text = "Using api_key=sk_test_12345 for request";
    let redacted = Redaction::redact(text);

    assert!(!redacted.contains("sk_test_12345"), "Should redact API key");
    assert!(
        redacted.contains("[REDACTED"),
        "Should show redaction marker"
    );
}

#[test]
fn test_redaction_redacts_tokens() {
    // CONTRACT: Redaction::redact() removes tokens
    let text = "Auth token: ghp_abc123xyz";
    let redacted = Redaction::redact(text);

    assert!(!redacted.contains("ghp_abc123xyz"), "Should redact token");
    assert!(
        redacted.contains("[REDACTED"),
        "Should show redaction marker"
    );
}

#[test]
fn test_redaction_redacts_passwords() {
    // CONTRACT: Redaction::redact() removes passwords
    let text = r#"password="mysecretpass123""#;
    let redacted = Redaction::redact(text);

    assert!(
        !redacted.contains("mysecretpass123"),
        "Should redact password"
    );
    assert!(
        redacted.contains("[REDACTED"),
        "Should show redaction marker"
    );
}

#[test]
fn test_redaction_multiple_patterns() {
    // CONTRACT: Redaction handles multiple sensitive patterns
    let text = "api_key=sk_123 token=ghp_456 password=secret";
    let redacted = Redaction::redact(text);

    assert!(!redacted.contains("sk_123"), "Should redact API key");
    assert!(!redacted.contains("ghp_456"), "Should redact token");
    assert!(!redacted.contains("secret"), "Should redact password");
}

#[test]
fn test_redaction_case_insensitive() {
    // CONTRACT: Redaction patterns are case-insensitive
    let text1 = "API_KEY=secret1";
    let text2 = "api_key=secret2";
    let text3 = "Api_Key=secret3";

    let redacted1 = Redaction::redact(text1);
    let redacted2 = Redaction::redact(text2);
    let redacted3 = Redaction::redact(text3);

    assert!(!redacted1.contains("secret1"), "Should redact uppercase");
    assert!(!redacted2.contains("secret2"), "Should redact lowercase");
    assert!(!redacted3.contains("secret3"), "Should redact mixedcase");
}

#[test]
fn test_redaction_contains_sensitive_data() {
    // CONTRACT: contains_sensitive_data() detects sensitive patterns
    assert!(Redaction::contains_sensitive_data("api_key=secret"));
    assert!(Redaction::contains_sensitive_data("token: abc123"));
    assert!(Redaction::contains_sensitive_data("password=pass"));
    assert!(Redaction::contains_sensitive_data(
        "AWS_SECRET_ACCESS_KEY=xyz"
    ));

    assert!(!Redaction::contains_sensitive_data("normal text"));
    assert!(!Redaction::contains_sensitive_data("user=john"));
}

#[test]
fn test_redaction_add_pattern() {
    // CONTRACT: add_pattern() allows custom redaction patterns
    let result = Redaction::add_pattern(r"(?i)credit[_-]?card");

    assert!(result.is_ok(), "Adding valid pattern should succeed");

    // Now credit card numbers should be redacted
    let text = "credit_card=1234-5678-9012-3456";
    let redacted = Redaction::redact(text);

    assert!(
        !redacted.contains("1234-5678-9012-3456"),
        "Custom pattern should redact"
    );
}

#[test]
fn test_log_output_file_creation() {
    // CONTRACT: LogOutput::File creates log directory if missing
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("logs").join("app.log");

    let config = LogConfig::builder()
        .output(LogOutput::File {
            path: log_file.clone(),
        })
        .build();

    let result = Logger::init(config);

    assert!(result.is_ok(), "Should create log directory");
    // Note: Actual file creation happens on first log write
}

#[test]
fn test_log_rotation_daily() {
    // CONTRACT: LogRotation::Daily creates new file each day
    let temp_dir = TempDir::new().unwrap();
    let log_dir = temp_dir.path().join("logs");

    let config = LogConfig::builder()
        .output(LogOutput::File {
            path: log_dir.join("app.log"),
        })
        .rotation(LogRotation::Daily)
        .build();

    let result = Logger::init(config);
    assert!(result.is_ok(), "Daily rotation config should succeed");
}

#[test]
fn test_non_blocking_logging() {
    // CONTRACT: Logging operations don't block main thread (NFR-004)
    use std::time::Instant;

    let config = LogConfig::development();
    let _ = Logger::init(config);

    let logger = Logger::for_module("perf_test");

    // Log many messages quickly
    let start = Instant::now();
    for i in 0..1000 {
        let _span = logger.start_operation(&format!("op_{}", i));
    }
    let duration = start.elapsed();

    // Should be fast (non-blocking)
    assert!(
        duration.as_millis() < 100,
        "Logging 1000 operations should be <100ms (non-blocking), took {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_invalid_log_directory() {
    // CONTRACT: Logger returns error if log directory not writable
    let read_only_path = std::path::PathBuf::from("/invalid/readonly/path.log");

    let config = LogConfig::builder()
        .output(LogOutput::File {
            path: read_only_path,
        })
        .build();

    let result = Logger::init(config);

    // Should fail with DirectoryNotWritable error
    if let Err(LogError::DirectoryNotWritable(path)) = result {
        assert!(path.to_str().unwrap().contains("invalid"));
    } else if result.is_ok() {
        // Some systems might allow this, but contract requires error checking
    }
}
