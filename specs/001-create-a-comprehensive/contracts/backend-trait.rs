// Backend Trait Contract
// This defines the interface all LLM backends must implement

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Core trait that all command generation backends must implement
#[async_trait]
pub trait CommandGenerator: Send + Sync {
    /// Generate a shell command from natural language input
    async fn generate_command(
        &self,
        request: &CommandRequest,
    ) -> Result<GeneratedCommand, GeneratorError>;
    
    /// Check if this backend is currently available for use
    async fn is_available(&self) -> bool;
    
    /// Get information about this backend's capabilities and performance
    fn backend_info(&self) -> BackendInfo;
    
    /// Perform any necessary cleanup when shutting down
    async fn shutdown(&self) -> Result<(), GeneratorError>;
}

/// Backend capability and performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendInfo {
    pub backend_type: BackendType,
    pub model_name: String,
    pub supports_streaming: bool,
    pub max_tokens: u32,
    pub typical_latency_ms: u64,
    pub memory_usage_mb: u64,
    pub version: String,
}

/// Errors that can occur during command generation
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum GeneratorError {
    #[error("Backend is not available: {reason}")]
    BackendUnavailable { reason: String },
    
    #[error("Request timeout after {timeout:?}")]
    Timeout { timeout: Duration },
    
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },
    
    #[error("Model generation failed: {details}")]
    GenerationFailed { details: String },
    
    #[error("Response parsing failed: {content}")]
    ParseError { content: String },
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Contract test interface - backends must pass these tests
pub trait BackendContractTests {
    /// Test basic command generation with simple request
    async fn test_basic_generation(&self);
    
    /// Test handling of malformed or empty requests
    async fn test_invalid_requests(&self);
    
    /// Test timeout behavior
    async fn test_timeout_handling(&self);
    
    /// Test concurrent request handling
    async fn test_concurrent_requests(&self);
    
    /// Test graceful degradation when backend becomes unavailable
    async fn test_unavailable_backend(&self);
}