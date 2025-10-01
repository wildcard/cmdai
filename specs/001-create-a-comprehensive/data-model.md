# Data Model Design

## Core Entities

### CommandRequest
**Purpose**: Represents user input and context for command generation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub input: String,                    // Natural language description
    pub context: Option<String>,          // Current directory, shell info
    pub shell: ShellType,                // Target shell (bash, zsh, fish, etc.)
    pub safety_level: SafetyLevel,       // User-configured safety setting
    pub backend_preference: Option<String>, // Preferred LLM backend
}
```

**Validation Rules**:
- `input` must be non-empty and <= 2000 characters
- `context` optional, <= 1000 characters if provided
- `shell` must be supported shell type
- `safety_level` must be valid enum value

### GeneratedCommand
**Purpose**: Contains LLM-generated command with metadata and safety assessment
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCommand {
    pub command: String,                  // Generated shell command
    pub explanation: String,              // Human-readable description
    pub safety_level: RiskLevel,         // Assessed risk level
    pub estimated_impact: Impact,         // File system/system impact
    pub alternatives: Vec<String>,        // Alternative command suggestions
    pub backend_used: String,             // Which backend generated this
    pub generation_time_ms: u64,          // Performance metric
    pub confidence_score: f32,            // Backend confidence (0.0-1.0)
}
```

**Validation Rules**:
- `command` must be valid shell syntax and <= 1000 characters
- `explanation` must be non-empty and <= 500 characters
- `safety_level` must match validation assessment
- `confidence_score` must be between 0.0 and 1.0

### SafetyRule
**Purpose**: Defines patterns and conditions for command safety validation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    pub id: String,                       // Unique rule identifier
    pub pattern: String,                  // Regex pattern to match
    pub risk_level: RiskLevel,            // Associated risk level
    pub message: String,                  // User warning message
    pub requires_confirmation: bool,       // Force confirmation prompt
    pub allow_override: bool,             // Can user override with --force
    pub category: SafetyCategory,         // Type of dangerous operation
}
```

**Validation Rules**:
- `pattern` must be valid regex expression
- `message` must be descriptive and actionable
- `category` must be valid enum value

### ModelBackendConfig
**Purpose**: Configuration for LLM backend instances
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBackendConfig {
    pub backend_type: BackendType,        // MLX, vLLM, Ollama, etc.
    pub endpoint: Option<String>,         // URL for remote backends
    pub model_path: Option<String>,       // Local model path
    pub model_name: String,               // Model identifier
    pub max_tokens: u32,                  // Generation limit
    pub temperature: f32,                 // Generation temperature
    pub timeout_seconds: u32,             // Request timeout
    pub enabled: bool,                    // Backend availability
}
```

### UserConfiguration
**Purpose**: User preferences and settings
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfiguration {
    pub safety_level: SafetyLevel,        // Default safety setting
    pub default_shell: ShellType,         // Preferred shell
    pub confirm_before_execution: bool,    // Always ask confirmation
    pub log_commands: bool,               // Enable command logging
    pub preferred_backends: Vec<String>,   // Backend priority order
    pub cache_directory: Option<PathBuf>,  // Custom cache location
    pub max_history_entries: u32,         // Command history limit
}
```

### CommandHistory
**Purpose**: Record of generated and executed commands for audit and learning
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandHistory {
    pub timestamp: DateTime<Utc>,         // When command was generated
    pub request: CommandRequest,          // Original user request
    pub generated_command: GeneratedCommand, // Generated command details
    pub executed: bool,                   // Whether user executed it
    pub execution_result: Option<String>, // Command output if executed
    pub user_feedback: Option<Feedback>,  // User rating/feedback
}
```

### ModelCache
**Purpose**: Local storage metadata for cached models
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCache {
    pub model_id: String,                 // HuggingFace model identifier
    pub local_path: PathBuf,              // Local storage path
    pub size_bytes: u64,                  // Model file size
    pub checksum: String,                 // SHA256 checksum
    pub version: String,                  // Model version
    pub last_used: DateTime<Utc>,         // For LRU eviction
    pub backend_compatibility: Vec<BackendType>, // Which backends can use
    pub performance_metrics: PerformanceMetrics, // Benchmarking data
}
```

## Enumerations

### RiskLevel
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Safe,        // Green - no risks identified
    Moderate,    // Yellow - minor risks, user should review
    High,        // Orange - significant risks, confirmation required
    Critical,    // Red - dangerous operation, explicit override needed
}
```

### SafetyLevel
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SafetyLevel {
    Strict,      // Block all moderate+ risk commands
    Moderate,    // Allow moderate, confirm high/critical
    Permissive,  // Allow high, confirm only critical
}
```

### BackendType
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BackendType {
    MLX,         // Apple Silicon optimized
    VLLM,        // High-performance inference server
    Ollama,      // Local model serving
    OpenAI,      // Remote API (future)
}
```

### ShellType
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Sh,
    PowerShell,  // Windows support
}
```

### SafetyCategory
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SafetyCategory {
    FileSystemDestruction,    // rm, rmdir operations
    DiskOperations,          // dd, mkfs, fdisk
    SystemModification,      // chmod, chown on system paths
    NetworkOperations,       // curl | sh, wget | sh
    ProcessManagement,       // killall, pkill
    PrivilegeEscalation,     // sudo, su operations
}
```

## Relationships

### Command Generation Flow
```
CommandRequest → Backend → GeneratedCommand → SafetyValidator → UserConfirmation → Execution
```

### Model Management
```
HuggingFace Hub → ModelCache → BackendConfig → Backend Instance
```

### Safety Validation
```
GeneratedCommand → SafetyRule[] → RiskAssessment → UserPrompt
```

### Configuration Hierarchy
```
CLI Args → Environment → User Config → System Config → Defaults → UserConfiguration
```

## State Transitions

### Command Processing States
- **Requested**: User provided natural language input
- **Generating**: Backend processing request
- **Generated**: Command produced, awaiting safety validation
- **Validated**: Safety check complete, awaiting user confirmation
- **Confirmed**: User approved execution
- **Executing**: Command running
- **Completed**: Execution finished (success/failure)
- **Rejected**: User declined execution or safety check failed

### Backend States
- **Available**: Backend ready for requests
- **Busy**: Processing request
- **Unavailable**: Backend down or unreachable
- **Error**: Backend encountered error

### Cache States
- **Fresh**: Recently downloaded, integrity verified
- **Stale**: Needs update check
- **Corrupt**: Failed integrity check, needs re-download
- **Missing**: Referenced but not found locally

## Performance Considerations

### Memory Usage
- CommandRequest: ~1KB typical
- GeneratedCommand: ~2KB typical
- SafetyRule: ~500B each
- ModelCache metadata: ~1KB per model
- CommandHistory: ~5KB per entry

### Storage Requirements
- Configuration: <10KB
- Safety rules database: ~100KB
- Command history (1000 entries): ~5MB
- Model cache metadata: ~1MB per 100 models

### Indexing Strategy
- SafetyRule: Hash map by pattern for O(1) lookup
- CommandHistory: Time-series index for efficient queries
- ModelCache: Hash map by model_id for fast access